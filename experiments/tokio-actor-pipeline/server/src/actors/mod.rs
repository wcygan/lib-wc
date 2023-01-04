use common::connection::Connection;

use crate::actors::processor::ProcessorHandle;
use crate::actors::reader::ReaderHandle;
use crate::actors::responder::ResponderHandle;

pub mod processor;
pub mod reader;
pub mod responder;

#[derive(Clone)]
pub struct ActorSystem {
    reader: ReaderHandle,
    _processor: ProcessorHandle,
    _responder: ResponderHandle,
}

impl ActorSystem {
    pub fn new() -> Self {
        let responder = ResponderHandle::new();
        let processor = ProcessorHandle::new(responder.clone());
        let reader = ReaderHandle::new(processor.clone());

        Self {
            reader,
            _processor: processor,
            _responder: responder,
        }
    }

    pub async fn handle(&mut self, conn: Connection) {
        self.reader.accept(conn).await;
    }
}
