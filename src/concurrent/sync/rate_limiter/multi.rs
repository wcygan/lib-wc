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
    /// Creates a new [`MultiRateLimiter`].
    pub fn new(period: Duration) -> Self {
        Self {
            period,
            rate_limiters: dashmap::DashMap::new(),
        }
    }

    /// Waits for the rate limiter to allow the client to send another query.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_wc::sync::MultiRateLimiter;
    /// use std::time::Duration;
    /// use std::sync::Arc;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let rate_limiter: Arc<MultiRateLimiter<u32>> = Arc::new(MultiRateLimiter::new(Duration::from_millis(50)));
    ///
    ///    futures::future::join_all(
    ///       (0..10).map(|x| {
    ///         let rate_limiter = rate_limiter.clone();
    ///         tokio::spawn(async move {
    ///          rate_limiter.throttle(x, || async {Ok::<(), anyhow::Error>(())}).await;
    ///           Ok::<(), anyhow::Error>(())
    ///         })
    ///      })
    ///    ).await;
    /// }
    /// ```    
    // async fn acquire(&self, key: K) -> Result<()> {
    //     let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter);
    //
    //     Retry::spawn(retry_strategy, || async {
    //         self.acquire_inner(key.clone()).await
    //     })
    //     .await
    // }

    /// Acquires the rate limiter for the given key.
    ///
    /// If the key does not exist, it is created.
    ///
    /// [`dashmap::DashMap::try_entry`] is used to avoid locking the entire map, so this
    /// operation is fallible and may need to be retried.
    // async fn acquire_inner(&self, key: K) -> Result<()> {
    //     match self.rate_limiters.try_entry(key.clone()) {
    //         Some(entry) => {
    //             let rate_limiter = entry.or_insert_with(|| RateLimiter::new(self.period));
    //             return rate_limiter.fff().await;
    //         }
    //         None => {}
    //     }
    //     Ok(())
    // }

    pub async fn throttle<Fut, F, T>(&self, key: K, f: F) -> Result<T>
    where
        Fut: std::future::Future<Output = T>,
        F: FnOnce() -> Fut,
    {
        loop {
            match self.rate_limiters.try_entry(key.clone()) {
                Some(entry) => {
                    let rate_limiter = entry.or_insert_with(|| RateLimiter::new(self.period));
                    return rate_limiter.value().throttle(f).await;
                }
                None => {}
            }
        }
    }
    //
    // async fn throttle_inner<Fut, F, T>(&self, key: K, f: &mut F) -> Result<T>
    // where
    //     Fut: std::future::Future<Output = T>,
    //     F: FnMut() -> Fut,
    // {
    //     match self.rate_limiters.try_entry(key.clone()) {
    //         Some(entry) => {
    //             let rate_limiter = entry.or_insert_with(|| RateLimiter::new(self.period));
    //             rate_limiter.value().throttle(f).await
    //         }
    //         None => Err(anyhow::anyhow!("Could not acquire rate limiter")),
    //     }
    // }

    // Take in an FnMut instead of an FnOnce so that we can retry the operation
    // pub async fn throttle<Fut, F, T>(&self, key: K, f: F) -> Result<T>
    // where
    //     Fut: std::future::Future<Output = Result<T>>,
    //     F: FnOnce() -> Fut,
    // {
    //     let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter);
    //
    //     Retry::spawn(retry_strategy, || async {
    //         self.throttle_inner(key.clone(), f).await
    //     })
    //     .await
    // }
    //
    // async fn throttle_inner<Fut, F, T>(&self, key: K, mut f: F) -> Result<T>
    // where
    //     Fut: std::future::Future<Output = Result<T>>,
    //     F: FnOnce() -> Fut,
    // {
    //     match self.rate_limiters.try_entry(key.clone()) {
    //         Some(entry) => {
    //             let rate_limiter = entry.or_insert_with(|| RateLimiter::new(self.period));
    //             rate_limiter.value().throttle(f).await?
    //         }
    //         None => Err(anyhow::anyhow!("Could not acquire rate limiter")),
    //     }
    // }
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
