use crate::actors::messages::{AddedValue, MultipliedValue};
use crate::BrokerType;
use actix::prelude::*;
use actix::{Actor, Context, Handler, ResponseActFuture};
use actix_broker::{BrokerIssue, BrokerSubscribe};

pub struct Multiplier;

impl Actor for Multiplier {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_async::<BrokerType, AddedValue>(ctx);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Multiplier stopped");
    }
}

impl Handler<AddedValue> for Multiplier {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: AddedValue, _ctx: &mut Self::Context) -> Self::Result {
        Box::pin(
            async move { multiply_by_two(msg.value).await }
                .into_actor(self)
                .map(|value, actor, _context| {
                    actor.issue_async::<BrokerType, MultipliedValue>(MultipliedValue { value })
                }),
        )
    }
}

async fn multiply_by_two(v: u32) -> u32 {
    v * 2
}
