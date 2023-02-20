use crate::sync::RateLimiter;
use anyhow::Result;
use crossbeam_utils::Backoff;
use std::hash::Hash;
use std::time::Duration;

pub struct MultiRateLimiter<K> {
    /// The period that each key is allowed to send a query
    period: Duration,

    /// The key-specific [`RateLimiter`]s
    rate_limiters: dashmap::DashMap<K, RateLimiter>,
}

impl<K: Eq + Hash + Clone> MultiRateLimiter<K> {
    /// Creates a new [`MultiRateLimiter`].
    pub fn new(period: Duration) -> Self {
        Self {
            period,
            rate_limiters: dashmap::DashMap::new(),
        }
    }

    pub async fn throttle<Fut, F, T>(&self, key: K, f: F) -> Result<T>
    where
        Fut: std::future::Future<Output = T>,
        F: FnOnce() -> Fut,
    {
        let mut retries = 10;
        let backoff = Backoff::new();

        loop {
            match self.rate_limiters.try_entry(key.clone()) {
                Some(entry) => {
                    let rate_limiter = entry.or_insert_with(|| RateLimiter::new(self.period));
                    return rate_limiter.value().throttle(f).await;
                }
                None => {
                    retries -= 1;
                    if retries == 0 {
                        return Err(anyhow::anyhow!("Failed to acquire rate limiter"));
                    }
                    backoff.spin();
                }
            }
        }
    }

    pub async fn throttle_mut<Fut, F, T>(&self, key: K, f: F) -> Result<T>
    where
        Fut: std::future::Future<Output = T>,
        F: FnMut() -> Fut,
    {
        let mut retries = 10;
        let backoff = Backoff::new();

        loop {
            match self.rate_limiters.try_entry(key.clone()) {
                Some(entry) => {
                    let rate_limiter = entry.or_insert_with(|| RateLimiter::new(self.period));
                    return rate_limiter.value().throttle_mut(f).await;
                }
                None => {
                    if retries == 0 {
                        return Err(anyhow::anyhow!("Failed to acquire rate limiter"));
                    }

                    retries -= 1;
                    backoff.spin();
                }
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::sync::Arc;
//     use std::time::Instant;
//
//     #[tokio::test]
//     async fn test_acquire_same_key_synchronously() -> Result<()> {
//         let rate_limiter = MultiRateLimiter::new(Duration::from_millis(10));
//
//         let start = Instant::now();
//
//         for _ in 0..10 {
//             rate_limiter.acquire("key").await?;
//         }
//
//         let elapsed = start.elapsed().as_millis();
//
//         assert!(elapsed >= 89);
//
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn test_acquire_same_key_synchronously2() -> Result<()> {
//         let period = 50;
//         let rate_limiter = MultiRateLimiter::new(Duration::from_millis(period));
//
//         let start = Instant::now();
//
//         rate_limiter.acquire("key").await?;
//         rate_limiter.acquire("key").await?;
//
//         let elapsed = start.elapsed().as_millis();
//
//         assert!(elapsed >= period as u128 - 1);
//
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn test_acquire_same_key_asynchronously() -> Result<()> {
//         let rate_limiter = Arc::new(MultiRateLimiter::new(Duration::from_millis(10)));
//
//         let start = Instant::now();
//
//         let futs = (0..10)
//             .map(|_| {
//                 let rate_limiter = rate_limiter.clone();
//                 tokio::spawn(async move {
//                     rate_limiter.acquire("key").await?;
//                     Ok::<(), anyhow::Error>(())
//                 })
//             })
//             .collect::<Vec<_>>();
//
//         futures::future::join_all(futs).await;
//
//         let elapsed = start.elapsed().as_millis();
//
//         assert!(elapsed > 0);
//         assert!(elapsed < 10);
//
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn test_multi_key_acquire_returns_immediately() -> Result<()> {
//         let period = 20;
//         let rate_limiter = Arc::new(MultiRateLimiter::new(Duration::from_millis(period)));
//
//         let start = Instant::now();
//
//         let futs = (0..200)
//             .map(|x| {
//                 let rate_limiter = rate_limiter.clone();
//                 tokio::spawn(async move {
//                     // This should return immediately
//                     rate_limiter.acquire(x).await?;
//                     Ok::<(), anyhow::Error>(())
//                 })
//             })
//             .collect::<Vec<_>>();
//
//         futures::future::join_all(futs).await;
//
//         let elapsed = start.elapsed().as_millis();
//
//         println!("elapsed: {}", elapsed);
//         assert!(elapsed > 0);
//         assert!(elapsed < period as u128);
//
//         Ok(())
//     }
//
//     #[tokio::test]
//     async fn test_multi_key_double_acquire_does_not_return_immediately() -> Result<()> {
//         let period = 20;
//         let rate_limiter = Arc::new(MultiRateLimiter::new(Duration::from_millis(period)));
//
//         let start = Instant::now();
//
//         let futs = (0..200)
//             .map(|x| {
//                 let rate_limiter = rate_limiter.clone();
//                 tokio::spawn(async move {
//                     // This should NOT return immediately since we're acquiring a permit for the same key twice
//                     rate_limiter.acquire(x).await?;
//                     rate_limiter.acquire(x).await?;
//                     Ok::<(), anyhow::Error>(())
//                 })
//             })
//             .collect::<Vec<_>>();
//
//         futures::future::join_all(futs).await;
//
//         let elapsed = start.elapsed().as_millis();
//
//         println!("elapsed: {}", elapsed);
//         assert!(elapsed > 0);
//         assert!(elapsed > period as u128);
//         assert!(elapsed < period as u128 * 2);
//
//         Ok(())
//     }
// }
