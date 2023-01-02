use std::time::Duration;

use crate::actors::{Processor, Responder};
use actix::{Actor, System};
use tokio::select;
use tracing::info;

mod actors;

/// Potentially use the actor model?
///  1. Listener    - accepts requestes   - (networking I/O)
///  2. Processor   - reads/writes to db  - (file I/O)
///  3. Responder   - responds to clients - (networking I/O)
#[actix::main]
async fn main() -> ServerResult<()> {
    set_up_logging()?;
    start_actors();
    accept_connections().await?;
    Ok(())
}

async fn accept_connections() -> ServerResult<()> {
    // TODO: replace the interval with a tcp listener
    let mut interval = tokio::time::interval(Duration::from_millis(250));

    info!("server is listening for connections");

    loop {
        select! {
            _ = interval.tick() => {
                info!("tick");
            }
            _ = tokio::signal::ctrl_c() => {
                info!("ctrl-c received, exiting");
                System::current().stop();
                break;
            }
        }
    }

    Ok(())
}

fn start_actors() {
    Processor.start();
    Responder.start();
}

fn set_up_logging() -> ServerResult<()> {
    tracing_subscriber::fmt::try_init().map_err(|_| ServerError::TracingInitializationError)
}

type ServerResult<T> = Result<T, ServerError>;

#[derive(Debug)]
enum ServerError {
    TracingInitializationError,
}
