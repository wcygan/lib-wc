use anyhow::{Context, Result};
use clap::Parser;
use load_generator_common::run;
use std::time::Duration;
use tokio::select;
use tokio::time::Instant;
use tracing::debug;

mod args;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = args::Cli::parse();
    match tracing_subscriber::fmt::try_init() {
        Ok(_) => {}
        Err(_) => {
            tracing_subscriber::fmt().init();
        }
    }

    run(cli.url, cli.connections, shutdown(cli.time))
        .await
        .context("error running load generator")
        .unwrap();

    Ok(())
}

async fn shutdown(time: Option<u16>) {
    match time {
        None => {
            debug!("Press Ctrl+C to stop");
            select! {
                _ = tokio::signal::ctrl_c() => {}
            }
        }
        Some(t) => {
            debug!("Running for {} seconds", t);
            debug!("Press Ctrl+C to stop early");
            let mut interval = tokio::time::interval_at(
                Instant::now() + Duration::from_secs(t.into()),
                Duration::from_secs(1),
            );
            select! {
                _ = interval.tick() => {}
                _ = tokio::signal::ctrl_c() => {}
            }
        }
    }
}
