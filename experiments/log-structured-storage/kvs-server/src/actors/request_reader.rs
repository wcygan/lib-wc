use actix::{Actor, Context, Handler};
use actix_broker::BrokerSubscribe;
use tracing::info;

use crate::server::BrokerType;

pub struct Reader;

impl Actor for Reader {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Reader has started processing messages")
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Reader has stopped processing messages")
    }
}
