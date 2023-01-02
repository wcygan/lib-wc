use actix::prelude::*;
use actix::{Actor, Context, Handler, ResponseActFuture};
use actix_broker::{BrokerIssue, BrokerSubscribe};

use crate::actors::messages::{AddedValue, NewValue};
use crate::BrokerType;

pub struct Adder;

impl Actor for Adder {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_async::<BrokerType, NewValue>(ctx);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Adder stopped");
    }
}

impl Handler<NewValue> for Adder {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: NewValue, _ctx: &mut Self::Context) -> Self::Result {
        Box::pin(
            async move { add_one(msg.value).await }
                .into_actor(self)
                .map(|value, actor, _context| {
                    actor.issue_async::<BrokerType, AddedValue>(AddedValue { value })
                }),
        )
    }
}

async fn add_one(v: u32) -> u32 {
    v + 1
}
