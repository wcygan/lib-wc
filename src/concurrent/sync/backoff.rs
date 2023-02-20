use rand::Rng;
use std::time::Duration;
use tokio::time::sleep;

pub(crate) struct Backoff {
    /// The current backoff duration.
    duration: Duration,
    /// The maximum backoff duration.
    max: Duration,
}

impl Backoff {
    /// Creates a new [`Backoff`].
    pub fn new() -> Self {
        Self {
            duration: Duration::from_micros(500),
            max: Duration::from_millis(256),
        }
    }

    /// Exponentially backs off
    pub async fn backoff(&mut self) {
        let random_time = rand::thread_rng().gen_range(0..50);
        sleep(self.duration + Duration::from_millis(random_time)).await;
        self.duration = (self.duration * 2).min(self.max);
    }
}
