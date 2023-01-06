use crate::actors::db_processor::DbProcessorHandle;
use kvs_common::connection::Connection;
use kvs_common::requests::Request;
use tokio::spawn;

#[derive(Debug, Clone)]
pub struct ReaderHandle {
    chan: tokio::sync::mpsc::Sender<Connection>,
}

struct ReaderActor {
    chan: tokio::sync::mpsc::Receiver<Connection>,
    db_processor: DbProcessorHandle,
}

impl ReaderHandle {
    pub fn new(db_processor: DbProcessorHandle) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        let actor = ReaderActor {
            chan: rx,
            db_processor,
        };

        spawn(async move { actor.run().await });

        Self { chan: tx }
    }

    pub async fn send(&mut self, conn: Connection) {
        self.chan.send(conn).await.unwrap();
    }
}

impl ReaderActor {
    async fn handle_connection(&mut self, mut conn: Connection) {
        let mut db_processor = self.db_processor.clone();
        spawn(async move {
            if let Ok(Some(request)) = conn.read::<Request>().await {
                println!("Got request: {:?}", request);
                db_processor.send(conn, request).await;
            }
        });
    }

    async fn run(mut self) {
        while let Some(conn) = self.chan.recv().await {
            self.handle_connection(conn).await;
        }
    }
}
