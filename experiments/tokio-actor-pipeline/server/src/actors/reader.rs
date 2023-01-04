use tokio::sync::{mpsc, oneshot};
use tracing::{error, info};

use common::connection::Connection;
use common::messages::Request;

use crate::actors::processor::ProcessorHandle;

#[derive(Clone)]
pub struct ReaderHandle {
    sender: mpsc::Sender<ReaderMessage>,
    next: ProcessorHandle,
}

struct ReaderMessage {
    conn: Connection,
    respond_to: oneshot::Sender<(Connection, Request)>,
}

struct ReaderActor {
    receiver: mpsc::Receiver<ReaderMessage>,
}

impl ReaderHandle {
    pub fn new(next: ProcessorHandle) -> Self {
        let (sender, receiver) = mpsc::channel(64);
        tokio::spawn(ReaderActor::new(receiver).run());
        Self { sender, next }
    }

    /// Accept a message into the actor's work queue
    pub async fn accept(&mut self, conn: Connection) {
        let (send, recv) = oneshot::channel();

        // Send a message to the actor
        let _ = self
            .sender
            .send(ReaderMessage {
                conn,
                respond_to: send,
            })
            .await;

        // Wait for the actor to respond
        match recv.await {
            Ok((connection, request)) => {
                self.next.accept(connection, request).await;
            }
            Err(_) => {
                error!("sender was dropped without sending");
            }
        };
    }
}

impl ReaderActor {
    fn new(receiver: mpsc::Receiver<ReaderMessage>) -> Self {
        Self { receiver }
    }

    /// Continuously processes messages from the receiver
    async fn run(mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await;
        }
    }

    /// Handle a specific message
    async fn handle_message(&mut self, mut msg: ReaderMessage) {
        let request = msg.conn.read::<Request>().await;
        match request {
            Ok(Some(request)) => {
                let _ = msg.respond_to.send((msg.conn, request));
            }
            Ok(None) => {
                info!("Connection closed");
            }
            Err(err) => {
                error!("Could not handle message: {:?}", err);
            }
        }
    }
}
