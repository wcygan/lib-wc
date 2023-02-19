use crate::sync::RateLimiter;
use anyhow::Result;
use std::hash::Hash;
use std::time::Duration;

pub struct MultiRateLimiter<K> {
    /// The period that each key is allowed to send a query
    period: Duration,

    /// The key-specific [`RateLimiter`]s
    rate_limiters: dashmap::DashMap<K, RateLimiter>,
}

impl<K: Eq + Hash + Clone> MultiRateLimiter<K> {
    pub fn new(period: Duration) -> Self {
        Self {
            period,
            rate_limiters: dashmap::DashMap::new(),
        }
    }

    pub async fn acquire(&self, key: K) -> Result<()> {
        let limiter = self
            .rate_limiters
            .entry(key)
            .or_insert_with(|| RateLimiter::new(self.period));

        limiter.acquire().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_multi_rate_limiter() -> Result<()> {
        let rate_limiter = MultiRateLimiter::new(Duration::from_millis(10));

        let start = Instant::now();

        for _ in 0..10 {
            rate_limiter.acquire("key").await?;
        }

        let elapsed = start.elapsed();

        assert!(elapsed >= Duration::from_millis(89));

        Ok(())
    }

    #[tokio::test]
    async fn test_multi_rate_limiter2() -> Result<()> {
        let rate_limiter = MultiRateLimiter::new(Duration::from_millis(50));

        let start = Instant::now();

        rate_limiter.acquire("key").await?;
        rate_limiter.acquire("key").await?;

        let elapsed = start.elapsed();

        assert!(elapsed >= Duration::from_millis(49));

        Ok(())
    }
}
