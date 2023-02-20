use anyhow::Result;
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
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::Mutex;
    /// use anyhow::Result;
    /// use std::time::Duration;
    /// use lib_wc::sync::RateLimiter;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     RateLimiter::new(Duration::from_millis(10));
    ///     Ok(())
    /// }
    /// ```
    pub fn new(period: Duration) -> Self {
        Self {
            interval: Mutex::new(interval(period)),
        }
    }

    /// Waits for the rate limiter to allow the client to send another query.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::Mutex;
    /// use anyhow::Result;
    /// use std::time::Duration;
    /// use lib_wc::sync::RateLimiter;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let rate_limiter = RateLimiter::new(Duration::from_millis(10));   
    ///
    ///     for _ in 0..1 {
    ///        rate_limiter.fff().await?;
    ///        // Send a query to a server
    ///     }
    ///
    ///    Ok(())
    /// }
    /// ```
    pub async fn fff(&self) -> Result<()> {
        let mut interval = self.interval.lock().await;
        interval.tick().await;
        Ok(())
    }

    pub async fn throttle<Fut, F, T>(&self, f: F) -> Result<T>
    where
        Fut: std::future::Future<Output = T>,
        F: FnOnce() -> Fut,
    {
        let mut interval = self.interval.lock().await;
        interval.tick().await;
        Ok(f().await)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_rate_limiter() -> Result<()> {
        let rate_limiter = RateLimiter::new(Duration::from_millis(10));

        let start = Instant::now();
        for _ in 0..10 {
            rate_limiter.fff().await?;
        }
        let end = start.elapsed().as_millis();

        assert!(end >= 89);
        Ok(())
    }

    #[tokio::test]
    async fn test_do_with_rate_limit() -> Result<()> {
        async fn hello() {}

        let rate_limiter = RateLimiter::new(Duration::from_millis(10));
        let current_time = Instant::now();
        let start = Instant::now();
        for _ in 0..10 {
            rate_limiter.throttle(hello).await?;
        }
        let end = start.elapsed().as_millis();

        assert!(end >= 89);
        Ok(())
    }

    #[tokio::test]
    async fn test_throttle_fn_that_does_nothing() -> Result<()> {
        let rate_limiter = RateLimiter::new(Duration::from_millis(10));

        let start = Instant::now();
        for _ in 0..10 {
            rate_limiter.throttle(|| async {}).await?;
        }
        let end = start.elapsed().as_millis();

        assert!(end >= 89);
        Ok(())
    }
}
