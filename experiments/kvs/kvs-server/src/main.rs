use kvs_common::{Error, DEFAULT_ADDRESS};
use tokio::net::TcpListener;
use tokio::select;
use tracing::info;

mod accept;
mod actors;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging()?;
    let listener = TcpListener::bind(DEFAULT_ADDRESS).await?;
    info!("Listening on {}", listener.local_addr()?);

    select! {
        _res = accept::accept_connections(listener) => {
            info!("Accept loop exited");
        },
        _ = tokio::signal::ctrl_c() => {
            info!("Ctrl-C received");
        }
    }

    Ok(())
}

fn setup_logging() -> Result<()> {
    tracing_subscriber::fmt::try_init().map_err(|_| Error::TracingInitializationError)
}

pub type Result<T> = kvs_common::Result<T>;
