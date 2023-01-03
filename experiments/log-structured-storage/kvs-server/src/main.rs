use crate::actors::{Database, Responder};
use actix::{Actor, System};
use std::future::Future;
use tokio::net::TcpListener;
use tokio::select;
use tracing::{error, info};

mod actors;
mod server;

#[actix::main]
async fn main() -> ServerResult<()> {
    set_up_logging()?;
    start_actors()?;
    run(tokio::signal::ctrl_c()).await?;
    Ok(())
}

async fn run(shutdown: impl Future) -> ServerResult<()> {
    let mut listener = server::Listener::new().await?;

    select! {
        res = listener.accept_connections() => {
            if let Err(err) = res {
                error!("{:?} encountered an error while accepting connections", err);
            }
        }
        _ = shutdown => {
            info!("ctrl-c received, exiting");
            System::current().stop();
        }
    }

    Ok(())
}

fn start_actors() -> ServerResult<()> {
    Database::new()?.start();
    Responder.start();
    Ok(())
}

fn set_up_logging() -> ServerResult<()> {
    tracing_subscriber::fmt::try_init().map_err(|_| ServerError::TracingInitializationError)
}

type ServerResult<T> = Result<T, ServerError>;

#[derive(Debug)]
pub enum ServerError {
    TracingInitializationError,
    DbError(std::io::Error),
}

impl From<std::io::Error> for ServerError {
    fn from(e: std::io::Error) -> Self {
        ServerError::DbError(e)
    }
}
