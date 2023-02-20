use anyhow::Result;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::{interval, Interval};

/// TODO
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

    /// TODO
    pub async fn throttle<Fut, F, T>(&self, f: F) -> Result<T>
    where
        Fut: std::future::Future<Output = T>,
        F: FnOnce() -> Fut,
    {
        self.wait().await;
        Ok(f().await)
    }

    async fn wait(&self) {
        let mut interval = self.interval.lock().await;
        interval.tick().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::time::Instant;

    #[tokio::test]
    async fn test_throttle_empty() -> Result<()> {
        let rate_limiter = RateLimiter::new(Duration::from_millis(10));
        let start = Instant::now();

        for _ in 0..10 {
            rate_limiter.throttle(|| async {}).await?;
        }

        let end = start.elapsed().as_millis();
        assert!(end >= 89);
        Ok(())
    }

    #[tokio::test]
    async fn test_throttle_fn() -> Result<()> {
        let rate_limiter = RateLimiter::new(Duration::from_millis(10));
        async fn hello() {
            println!("Hello, world!")
        }

        let start = Instant::now();
        for _ in 0..10 {
            rate_limiter.throttle(hello).await?;
        }
        let end = start.elapsed().as_millis();

        assert!(end >= 89);
        Ok(())
    }

    #[tokio::test]
    async fn test_throttle_with_mutable_data() -> Result<()> {
        let rate_limiter = Arc::new(RateLimiter::new(Duration::from_millis(10)));
        let data = Arc::new(Mutex::new(0));

        async fn hello(data: Arc<Mutex<i32>>) {
            let mut data = data.lock().await;
            *data += 1;
        }

        let start = Instant::now();
        let futs = (0..10).map(|_| {
            let data = data.clone();
            let rate_limiter = rate_limiter.clone();
            tokio::spawn(async move {
                rate_limiter.throttle(|| hello(data.clone())).await?;
                Ok::<(), anyhow::Error>(())
            })
        });

        for fut in futs {
            fut.await??;
        }

        let end = start.elapsed().as_millis();
        let data = data.lock().await;
        assert!(end >= 89);
        assert_eq!(*data, 10);
        Ok(())
    }

    #[tokio::test]
    async fn test_throttle_fn_mut_with_mutable_data_2() -> Result<()> {
        let data = Arc::new(Mutex::new(Data { data: 0 }));
        let rate_limiter = Arc::new(RateLimiter::new(Duration::from_millis(10)));

        struct Data {
            data: i32,
        }

        impl Data {
            async fn increment(&mut self) {
                self.data += 1;
            }
        }

        async fn hello(data: Arc<Mutex<Data>>) {
            let mut data = data.lock().await;
            data.increment().await;
        }

        let start = Instant::now();
        let futs = (0..10).map(|_| {
            let data = data.clone();
            let rate_limiter = rate_limiter.clone();
            tokio::spawn(async move {
                rate_limiter.throttle(|| hello(data.clone())).await?;
                Ok::<(), anyhow::Error>(())
            })
        });

        for fut in futs {
            fut.await??;
        }

        let end = start.elapsed().as_millis();
        let data = data.lock().await;
        assert!(end >= 89);
        assert_eq!(data.data, 10);
        Ok(())
    }
}
