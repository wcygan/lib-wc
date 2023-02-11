use tokio::select;
use tokio::sync::mpsc;
use tokio::time::Instant;

use crate::shutdown::Shutdown;

pub static REQUESTS_PER_UPDATE: u64 = 10000;

pub type Update = ();

pub(crate) async fn progress_tracker(mut rx: mpsc::Receiver<Update>, mut shutdown: Shutdown) {
    let spinner = indicatif::ProgressBar::new_spinner();
    let mut text_update = tokio::time::interval_at(
        Instant::from(std::time::Instant::now() + std::time::Duration::from_millis(500)),
        std::time::Duration::from_millis(500),
    );
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(550));

    let start = std::time::Instant::now();
    let mut requests: u64 = 0;

    while !shutdown.is_shutdown() {
        spinner.tick();
        select! {
            _ = text_update.tick() => {
                let elapsed = start.elapsed().as_secs_f64();
                let rate = requests as f64 / elapsed;
                let msg = format!("{:.2} requests per second", rate);
                spinner.set_message(msg);
            }
            _ = interval.tick() => {  }
            _ = shutdown.recv() => { break }
            Some(_) = rx.recv() => { requests += REQUESTS_PER_UPDATE }
        }
    }

    spinner.finish()
}
