use crate::actors::messages::MultipliedValue;
use crate::BrokerType;
use actix::{Actor, Context, Handler};
use actix_broker::BrokerSubscribe;

pub struct Sink;

impl Actor for Sink {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_async::<BrokerType, MultipliedValue>(ctx);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Sink stopped");
    }
}

impl Handler<MultipliedValue> for Sink {
    type Result = ();

    fn handle(&mut self, msg: MultipliedValue, _ctx: &mut Self::Context) -> Self::Result {
        println!("{}", msg.value);
    }
}
