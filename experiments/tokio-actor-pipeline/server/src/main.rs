use tokio::net::TcpListener;
use tokio::select;
use tracing::info;

use accept::accept_connections;
use common::ADDRESS;

use crate::actors::ActorSystem;

mod accept;
mod actors;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging()?;
    let listener = TcpListener::bind(ADDRESS).await?;
    let system = ActorSystem::new();
    info!("Listening on {}", ADDRESS);

    select! {
        res = accept_connections(listener, system) => {
            info!("stopped accepting connections: {:?}", res);
        }
        _ = tokio::signal::ctrl_c() => {
            info!("ctrl+c encountered")
        }
    }

    Ok(())
}

fn setup_logging() -> Result<()> {
    tracing_subscriber::fmt::try_init().map_err(|_| Error::TracingInitializationError)
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    TracingInitializationError,
    IoError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}
