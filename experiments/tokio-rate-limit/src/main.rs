use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use tokio::{select, spawn};

#[derive(Parser)]
struct Cli {
    #[arg(short = 'q', long, value_name = "qps", required = true)]
    qps: u64,
    #[arg(short = 'c', long, value_name = "concurrency", required = true)]
    concurrency: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let (tx, rx) = tokio::sync::mpsc::channel::<()>(1000);
    let limiter = Arc::new(lib_wc::sync::RateLimiter::new(cli.qps)?);

    for _ in 0..cli.concurrency {
        let limiter = limiter.clone();
        let tx = tx.clone();
        spawn(run(limiter, tx));
    }

    spawn(listen(rx));

    select! {
        _ = tokio::signal::ctrl_c() => {
            println!("Ctrl-C received, exiting");
        }
    }

    Ok(())
}

async fn run(
    limiter: Arc<lib_wc::sync::RateLimiter>,
    tx: tokio::sync::mpsc::Sender<()>,
) -> Result<()> {
    loop {
        limiter.acquire().await?;
        tx.send(()).await?;
    }
}

async fn listen(mut rx: tokio::sync::mpsc::Receiver<()>) {
    let mut elapsed = tokio::time::Instant::now();
    let mut interval = tokio::time::interval_at(
        tokio::time::Instant::now() + std::time::Duration::from_millis(250),
        std::time::Duration::from_secs(1),
    );
    let mut count = 0;
    loop {
        tokio::select! {
            _ = interval.tick() => {
                let qps = count as f64 / elapsed.elapsed().as_secs_f64();
                println!("QPS: {}", qps);
                elapsed = tokio::time::Instant::now();
                count = 0;
            }
            _ = rx.recv() => {
                count += 1;
            }
        }
    }
}
