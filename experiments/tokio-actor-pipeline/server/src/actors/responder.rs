use tokio::sync::mpsc;

use common::connection::Connection;
use common::messages::Response;

#[derive(Clone)]
pub struct ResponderHandle {
    sender: mpsc::Sender<ResponderMessage>,
}

struct ResponderActor {
    receiver: mpsc::Receiver<ResponderMessage>,
}

struct ResponderMessage {
    conn: Connection,
    response: Response,
}

impl ResponderHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(64);
        tokio::spawn(ResponderActor::new(receiver).run());
        Self { sender }
    }

    pub async fn accept(&mut self, conn: Connection, response: Response) {
        let _ = self.sender.send(ResponderMessage { conn, response }).await;
    }
}

impl ResponderActor {
    fn new(receiver: mpsc::Receiver<ResponderMessage>) -> Self {
        Self { receiver }
    }

    /// Continuously processes messages from the receiver
    async fn run(mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await;
        }
    }

    /// Handle a specific message. Spawns a new task to handle the response.
    async fn handle_message(&mut self, mut msg: ResponderMessage) {
        tokio::spawn(async move {
            let _ = msg.conn.write::<Response>(&msg.response).await;
        });
    }
}
