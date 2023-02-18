use anyhow::{anyhow, Result};
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::{interval, Interval};

/// A client-side rate limiter. This is useful for limiting the number of queries sent to a server
/// from a single client. For example, it is useful inside of a web crawler to limit the number of
/// requests sent by the crawler.
///
/// The rate limit is a "best effort" rate limit. It is not guaranteed that the rate limit will be
/// exactly the specified number of queries per second. It is possible that the rate limit will be
/// exceeded by a small amount.
pub struct RateLimiter {
    /// The maximum allowed number of queries per second
    max_qps: f64,

    /// The mutex that will be locked when the rate limiter is waiting for the interval to tick.
    ///
    /// It's important to use a tokio::sync::Mutex here instead of a std::sync::Mutex. The reason is
    /// that the tokio::sync::Mutex does not block & the MutexGuard is held across await points.
    ///
    /// If you tried to use std::sync::Mutex instead, you would get a compiler error when
    /// spawning tokio tasks because the MutexGuard would not be Send.
    interval: Mutex<Interval>,
}

impl RateLimiter {
    /// Creates a new rate limiter.
    ///
    /// Returns an error if the max QPS is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::Mutex;
    /// use anyhow::Result;
    /// use lib_wc::sync::RateLimiter;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let max_qps = 100.0;
    ///     RateLimiter::new(max_qps)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(max_qps: f64) -> Result<Self> {
        // Make sure that the max QPS is not close to 0
        if max_qps < 0.000001 {
            return Err(anyhow!("The max QPS must be greater than 0"));
        }

        let interval_secs_f64 = 1_f64 / max_qps;

        Ok(Self {
            max_qps,
            interval: Mutex::new(interval(Duration::from_secs_f64(interval_secs_f64))),
        })
    }

    /// Waits for the rate limiter to allow the client to send another query.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::Mutex;
    /// use anyhow::Result;
    /// use lib_wc::sync::RateLimiter;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let max_qps = 1.0;
    ///     let rate_limiter = RateLimiter::new(max_qps)?;   
    ///
    ///     for _ in 0..1 {
    ///        rate_limiter.acquire().await?;
    ///        // Send a query to a server
    ///     }
    ///
    ///    Ok(())
    /// }
    /// ```
    pub async fn acquire(&self) -> Result<()> {
        let mut interval = self.interval.lock().await;
        interval.tick().await;
        Ok(())
    }
}
