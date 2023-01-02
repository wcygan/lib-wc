use std::time::Duration;

use actix::{Actor, System};
use actix_broker::{Broker, SystemBroker};
use tokio::select;

use crate::actors::messages::Tick;
use crate::actors::{Adder, Multiplier, Sink, ValueGenerator};

mod actors;

type BrokerType = SystemBroker;

#[actix::main]
async fn main() {
    ValueGenerator.start();
    Adder.start();
    Multiplier.start();
    Sink.start();
    interval_loop(Duration::from_millis(500)).await;
}

/// Repeatedly send ticks on an interval
async fn interval_loop(duration: Duration) {
    let mut interval = tokio::time::interval(duration);

    loop {
        select! {
            _ = interval.tick() => {
                Broker::<BrokerType>::issue_async(Tick);
            }
            _ = tokio::signal::ctrl_c() => {
                println!("Ctrl-C was pressed! Stopping!");
                System::current().stop();
                break;
            }
        }
    }
}
