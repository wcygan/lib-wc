use std::future::Future;

use anyhow::{Context, Result};
use tokio::select;
use tokio::sync::{broadcast, mpsc};
use tracing::debug;

use crate::client::Connection;
use crate::shutdown::Shutdown;
use crate::updates::{progress_tracker, Update};

pub async fn run(url: String, connections: u16, shutdown: impl Future) -> Result<()> {
    let (notify_shutdown, _) = broadcast::channel::<()>(1);
    let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel::<()>(1);
    let (update_tx, update_rx) = mpsc::channel::<Update>(100);

    debug!("Starting {} connections", connections);

    for _ in 0..connections {
        let mut client = Connection::new(
            url.clone(),
            update_tx.clone(),
            Shutdown::new(notify_shutdown.subscribe()),
            shutdown_complete_tx.clone(),
        )
        .await
        .context("error building client")?;

        tokio::spawn(async move {
            client.run().await;
        });
    }

    tokio::spawn({
        let sd = Shutdown::new(notify_shutdown.subscribe());
        async move {
            progress_tracker(update_rx, sd).await;
        }
    });

    select! {
        _ = shutdown => {
            debug!("Shutting down");
        }
    }

    signal_shutdown(notify_shutdown);
    wait_for_task_to_finish(shutdown_complete_tx, shutdown_complete_rx).await;

    debug!("Done");

    Ok(())
}

/// Sends a shutdown signal to all tasks listening for it.
fn signal_shutdown(notify_shutdown: broadcast::Sender<()>) {
    debug!("Sending shutdown signal");
    drop(notify_shutdown);
}

/// Waits for all tasks to finish after the shutdown signal has been sent.
async fn wait_for_task_to_finish(tx: mpsc::Sender<()>, mut rx: mpsc::Receiver<()>) {
    debug!("Waiting for tasks to finish");
    drop(tx);
    let _ = rx.recv().await;
}
