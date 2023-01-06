use kvs_common::connection::Connection;
use kvs_common::requests::Response;
use tokio::spawn;

#[derive(Debug, Clone)]
pub struct WriterHandle {
    chan: tokio::sync::mpsc::Sender<WriterMessage>,
}

struct WriterActor {
    chan: tokio::sync::mpsc::Receiver<WriterMessage>,
}

#[derive(Debug)]
struct WriterMessage {
    conn: Connection,
    response: Response,
}

impl WriterHandle {
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        let actor = WriterActor { chan: rx };

        spawn(async move { actor.run().await });

        Self { chan: tx }
    }

    pub async fn send(&mut self, conn: Connection, response: Response) {
        self.chan
            .send(WriterMessage { conn, response })
            .await
            .unwrap();
    }
}

impl WriterActor {
    async fn process(&mut self, mut msg: WriterMessage) {
        spawn(async move {
            let _ = msg.conn.write::<Response>(&msg.response).await;
        });
    }

    async fn run(mut self) {
        while let Some(msg) = self.chan.recv().await {
            self.process(msg).await;
        }
    }
}
