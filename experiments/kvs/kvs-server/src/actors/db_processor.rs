use crate::actors::writer::WriterHandle;
use kvs_common::connection::Connection;
use kvs_common::requests::{Request, Response};
use std::io::Error;

use tokio::spawn;

#[derive(Debug, Clone)]
pub struct DbProcessorHandle {
    chan: tokio::sync::mpsc::Sender<DbProcessorMessage>,
}

struct DbProcessorActor {
    kv_store: kvs::KVStore,
    chan: tokio::sync::mpsc::Receiver<DbProcessorMessage>,
    writer: WriterHandle,
}

#[derive(Debug)]
struct DbProcessorMessage {
    conn: Connection,
    request: Request,
}

impl DbProcessorHandle {
    pub fn new(writer: WriterHandle) -> crate::Result<Self> {
        let kv_store = kvs::KVStore::open()?;

        let (tx, rx) = tokio::sync::mpsc::channel(100);

        let actor = DbProcessorActor {
            kv_store,
            chan: rx,
            writer,
        };

        spawn(async move { actor.run().await });

        Ok(Self { chan: tx })
    }

    pub async fn send(&mut self, conn: Connection, request: Request) {
        self.chan
            .send(DbProcessorMessage { conn, request })
            .await
            .unwrap();
    }
}

impl DbProcessorActor {
    async fn process(&mut self, msg: DbProcessorMessage) {
        let res: Result<Response, Error> = match msg.request {
            Request::Put { key, value } => {
                match self.kv_store.insert(key.as_bytes(), value.as_bytes()) {
                    Ok(_) => Ok(Response::Ok),
                    Err(e) => Err(e),
                }
            }
            Request::Get { key } => match self.kv_store.get(key.as_bytes()) {
                Ok(Some(v)) => {
                    let string = String::from_utf8_lossy(&v);
                    let trim = string.trim();
                    match trim {
                        // empty string means that the entry was removed, hence the key doesn't exist
                        "" => Ok(Response::KeyNotFound),
                        _ => Ok(Response::OkWithValue {
                            value: string.to_string(),
                        }),
                    }
                }
                Ok(None) => Ok(Response::KeyNotFound),
                Err(e) => Err(e),
            },
            Request::Update { key, value } => {
                match self.kv_store.update(key.as_bytes(), value.as_bytes()) {
                    Ok(_) => Ok(Response::Ok),
                    Err(e) => Err(e),
                }
            }
            Request::Delete { key } => match self.kv_store.delete(key.as_bytes()) {
                Ok(_) => Ok(Response::Ok),
                Err(e) => Err(e),
            },
        };

        match res {
            Ok(value) => {
                let mut writer = self.writer.clone();
                spawn(async move {
                    writer.send(msg.conn, value).await;
                });
            }
            Err(_) => {}
        }
    }

    async fn run(mut self) {
        while let Some(msg) = self.chan.recv().await {
            self.process(msg).await;
        }
    }
}
