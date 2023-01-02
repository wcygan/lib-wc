use actix::{Actor, Context, Handler};
use kvs_common::requests::Response;
use tracing::info;

pub struct Responder;

impl Actor for Responder {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Responder has started processing messages")
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Responder has stopped processing messages")
    }
}

impl Handler<Response> for Responder {
    type Result = ();

    fn handle(&mut self, msg: Response, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}
