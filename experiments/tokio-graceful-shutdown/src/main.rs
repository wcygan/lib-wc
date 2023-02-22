use std::time::Duration;

use lib_wc::sync::{ShutdownController, ShutdownListener};
use tokio::select;
use tokio::signal::ctrl_c;
use tokio::time::{interval, interval_at};

/// Graceful Shutdown (https://tokio.rs/tokio/topics/shutdown)
///
/// A clear implementation of graceful shutdown using tokio::sync::broadcast
/// and tokio::sync::mpsc.
///
/// This example spawns 10 tasks that do work until being told to stop.
///
/// After 2 seconds, or when an interrupt signal is received, the main task sends
/// a shutdown signal to all tasks.
///
/// Each task receives the shutdown signal and exits gracefully.
///
/// The main task waits for all tasks to finish and then exits.
#[tokio::main]
async fn main() {
    let shutdown = ShutdownController::new();

    // Spawn 10 tens
    for i in 0..10 {
        let shutdown = shutdown.subscribe();
        tokio::spawn(async move {
            let mut task = Task::new(i, shutdown);
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
    shutdown.shutdown().await;
    println!("shutdown complete");
}

struct Task {
    /// The Task id
    id: u32,

    /// Shutdown signal that is used to signal that the task should stop
    shutdown: ShutdownListener,
}

impl Task {
    pub fn new(id: u32, shutdown: ShutdownListener) -> Self {
        Self { id, shutdown }
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
                    if self.id % 2 == 0 {
                        tokio::time::sleep(Duration::from_millis(850)).await;
                        println!("{}: long tick", self.id)
                    } else {
                        println!("{} tick", self.id);
                    }
                }
            }
        }
    }
}
