use std::time::Duration;

use lib_wc::sync::Shutdown;
use tokio::select;
use tokio::signal::ctrl_c;
use tokio::sync::{broadcast, mpsc};
use tokio::time::{interval, interval_at};

/// Graceful Shutdown (https://tokio.rs/tokio/topics/shutdown)
///
/// A clear implementation of graceful shutdown using tokio::sync::broadcast
/// and tokio::sync::mpsc.
///
/// This example spawns 10 tasks that each print a message every second.
///
/// After 2 seconds, or when an interrupt signal is received, the main task sends
/// a shutdown signal to all tasks.
///
/// Each task receives the shutdown signal and exits gracefully.
///
/// The main task waits for all tasks to finish and then exits.
#[tokio::main]
async fn main() {
    let (notify_shutdown, _) = broadcast::channel::<()>(1);
    let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel::<()>(1);

    // Spawn 10 tens
    for i in 0..10 {
        let shutdown = Shutdown::new(notify_shutdown.subscribe());
        let shutdown_complete_tx = shutdown_complete_tx.clone();
        tokio::spawn(async move {
            let mut task = Task::new(i, shutdown, shutdown_complete_tx);
            task.run().await;
        });
    }

    // Wait for 2 second
    let mut interval = interval_at(
        tokio::time::Instant::now() + Duration::from_secs(2),
        Duration::from_secs(1),
    );

    // Block until ctrl-c or 2 seconds pass, whichever comes first
    select! {
        _ = ctrl_c() => {
            println!("ctrl-c received");
        }
        _ = interval.tick() => {
            println!("2 seconds passed");
        }
    }

    println!("shutdown starting");

    // Send shutdown signal to all tasks
    drop(notify_shutdown);

    // Wait for all tasks to finish
    //    Note:  We need to drop `shutdown_complete_tx` here
    //           because otherwise the `recv` will never return.
    drop(shutdown_complete_tx);
    let _ = shutdown_complete_rx.recv().await;

    println!("shutdown complete");
}

struct Task {
    /// The Task id
    id: u32,

    /// Shutdown signal that is used to signal that the task should stop
    shutdown: Shutdown,

    /// Implicitly used to signal that the task has finished by being dropped
    _shutdown_complete_tx: mpsc::Sender<()>,
}

impl Task {
    pub fn new(id: u32, shutdown: Shutdown, _shutdown_complete_tx: mpsc::Sender<()>) -> Self {
        Self {
            id,
            shutdown,
            _shutdown_complete_tx,
        }
    }

    /// Simulate some work
    pub async fn run(&mut self) {
        let mut interval = interval(Duration::from_millis(900));
        while !self.shutdown.is_shutdown() {
            select! {
                _ = self.shutdown.recv() => {
                    println!("{} is shutting down", self.id);
                    return;
                }
                _ = interval.tick() => {
                    println!("{} ticked", self.id);
                }
            }
        }
    }
}
