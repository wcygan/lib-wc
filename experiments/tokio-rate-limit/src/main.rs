use anyhow::Result;
use clap::Parser;
use lib_wc::sync::MultiRateLimiter;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::{select, spawn, sync::mpsc};

#[derive(Parser)]
struct Cli {
    /// The period with which to rate limit
    #[arg(short = 'p', long, value_name = "period", default_value = "500")]
    period_ms: u64,

    /// The number of clients to spawn
    #[arg(short = 'c', long, value_name = "clients", default_value = "64")]
    clients: u64,

    /// The radix which is used to spread clients across different sets of keys.
    /// A smaller radix will cause a higher level of contention
    /// between clients.
    #[arg(short = 'r', long, value_name = "radix", default_value = "4")]
    radix: u8,
}

type Key = u64;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let period = Duration::from_millis(cli.period_ms);
    let limiter: Arc<MultiRateLimiter<Key>> = Arc::new(MultiRateLimiter::new(period));

    let radix = if cli.clients < cli.radix as u64 {
        cli.clients
    } else {
        cli.radix as u64
    };

    let tx_map: HashMap<Key, mpsc::Sender<Key>> = (0..radix)
        .map(|i| {
            let key = i as Key;
            let (tx, rx) = mpsc::channel(1000);
            spawn(listen(key, rx, period));
            (key, tx)
        })
        .collect();

    for i in 0..cli.clients {
        let target = i % radix;
        let tx = tx_map.get(&target).unwrap().clone();
        spawn(
            Client {
                target,
                limiter: limiter.clone(),
                tx,
            }
            .run(),
        );
    }

    select! {
        _ = tokio::signal::ctrl_c() => {
            println!("Ctrl-C received, exiting");
        }
    }

    Ok(())
}

struct Client {
    target: Key,
    limiter: Arc<MultiRateLimiter<Key>>,
    tx: mpsc::Sender<Key>,
}

impl Client {
    async fn run(self) -> Result<()> {
        let Client {
            target,
            limiter,
            tx,
        } = self;

        loop {
            let _ = limiter
                .throttle(target, || Client::send(target, tx.clone()))
                .await?;
        }
    }

    async fn send(target: Key, tx: mpsc::Sender<Key>) -> Result<()> {
        tx.send(target).await?;
        Ok(())
    }
}

async fn listen(key: Key, mut rx: tokio::sync::mpsc::Receiver<Key>, period: Duration) {
    let elapsed = tokio::time::Instant::now();

    let mut interval = tokio::time::interval_at(
        tokio::time::Instant::now() + period,
        Duration::from_millis(500),
    );

    let mut count = 0;

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let qps = count as f64 / elapsed.elapsed().as_secs_f64();
                println!("[Key {key} QPS] {qps} ");
            }
            _ = rx.recv() => {
                count += 1;
            }
        }
    }
}
