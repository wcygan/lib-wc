use std::sync::atomic::AtomicU32;

use tokio::sync::{mpsc, oneshot};
use tracing::{error, info};

use common::connection::Connection;
use common::messages::{Action, Request, Response};

use crate::actors::responder::ResponderHandle;

#[derive(Clone)]
pub struct ProcessorHandle {
    sender: mpsc::Sender<ProcessorMessage>,
    next: ResponderHandle,
}

struct ProcessorMessage {
    conn: Connection,
    request: Request,
    respond_to: oneshot::Sender<(Connection, Response)>,
}

struct ProcessorActor {
    receiver: mpsc::Receiver<ProcessorMessage>,
}

impl ProcessorHandle {
    pub fn new(next: ResponderHandle) -> Self {
        let (sender, receiver) = mpsc::channel(64);
        tokio::spawn(ProcessorActor::new(receiver).run());
        Self { sender, next }
    }

    /// Accept a message into the actor's work queue
    pub async fn accept(&mut self, conn: Connection, request: Request) {
        let (send, recv) = oneshot::channel();

        // Send a message to the actor
        let _ = self
            .sender
            .send(ProcessorMessage {
                conn,
                request,
                respond_to: send,
            })
            .await;

        // Wait for the actor to respond
        match recv.await {
            Ok((connection, response)) => {
                self.next.accept(connection, response).await;
            }
            Err(_) => {
                error!("sender was dropped without sending");
            }
        }
    }
}

impl ProcessorActor {
    fn new(receiver: mpsc::Receiver<ProcessorMessage>) -> Self {
        Self { receiver }
    }

    /// Continuously processes messages from the receiver
    async fn run(mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await;
        }
    }

    /// Handle a specific message
    async fn handle_message(&mut self, msg: ProcessorMessage) {
        let response = respond(msg.request).await;
        let _ = msg.respond_to.send((msg.conn, response));
    }
}

/// This method simulates an async computation
async fn respond(request: Request) -> Response {
    static PING_COUNTER: AtomicU32 = AtomicU32::new(0);
    static PONG_COUNTER: AtomicU32 = AtomicU32::new(0);

    match request {
        Request {
            action: Action::Ping,
            ..
        } => {
            let count = PING_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            info!("Ping count increased to {}", count);
            request.action.response(count)
        }
        Request {
            action: Action::Pong,
            ..
        } => {
            let count = PONG_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            info!("Pong count increased to {}", count);
            request.action.response(count)
        }
    }
}
