use crate::ServerError;
use actix::{Actor, Context, Handler};
use kvs::KVStore;
use kvs_common::requests::{Request, Response};
use std::path::Path;
use tracing::info;

pub struct Database {
    db: KVStore,
}

impl Database {
    pub fn new() -> Result<Self, ServerError> {
        let db = KVStore::open(Path::new("db"))?;
        Ok(Database { db })
    }
}

impl Actor for Database {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Database has started processing messages")
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Database has stopped processing messages")
    }
}

impl Handler<Request> for Database {
    type Result = ();

    fn handle(&mut self, msg: Request, ctx: &mut Self::Context) -> Self::Result {
        println!("Processor received message: {:?}", msg);
    }
}

fn process(request: Request) -> Response {
    todo!()
}
