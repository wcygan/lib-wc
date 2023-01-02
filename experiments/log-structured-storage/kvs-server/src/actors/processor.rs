use actix::{Actor, Context, Handler};
use kvs_common::requests::{Request, Response};
use tracing::info;

pub struct Processor;

impl Actor for Processor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Processor has started processing messages")
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Processor has stopped processing messages")
    }
}

impl Handler<Request> for Processor {
    type Result = ();

    fn handle(&mut self, msg: Request, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}

fn process(request: Request) -> Response {
    todo!()
}
