use crate::actors::db_processor::DbProcessorHandle;
use crate::actors::reader::ReaderHandle;
use crate::actors::writer::WriterHandle;
use kvs_common::connection::Connection;

#[derive(Clone)]
pub struct System {
    reader: ReaderHandle,
}

impl System {
    pub fn new() -> crate::Result<Self> {
        let writer = WriterHandle::new();
        let db_processor = DbProcessorHandle::new(writer)?;
        let reader = ReaderHandle::new(db_processor);
        Ok(Self { reader })
    }

    pub async fn handle_connection(&mut self, conn: Connection) {
        self.reader.send(conn).await;
    }
}
