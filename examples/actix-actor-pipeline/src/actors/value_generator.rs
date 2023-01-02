use std::sync::atomic::{AtomicU32, Ordering};

use actix::prelude::*;
use actix::{Actor, Context, Handler, ResponseActFuture};
use actix_broker::{BrokerIssue, BrokerSubscribe};

use crate::actors::messages::{NewValue, Tick};
use crate::BrokerType;

pub struct ValueGenerator;

impl Actor for ValueGenerator {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_async::<BrokerType, Tick>(ctx);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("ValueGenerator stopped");
    }
}

impl Handler<Tick> for ValueGenerator {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, _msg: Tick, _ctx: &mut Self::Context) -> Self::Result {
        Box::pin(async { async_computation().await }.into_actor(self).map(
            |value, actor, _context| actor.issue_async::<BrokerType, NewValue>(NewValue { value }),
        ))
    }
}

/// This simulates some async computation
async fn async_computation() -> u32 {
    static GENERATOR: AtomicU32 = AtomicU32::new(0);
    GENERATOR.fetch_add(1, Ordering::Acquire)
}
