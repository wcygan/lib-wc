use crate::sync::backoff::Backoff;
use crate::sync::RateLimiter;
use std::hash::Hash;
use std::time::Duration;

/// [`MultiRateLimiter`] enables key-based rate limiting, where each key has its own [`RateLimiter`].
///
/// This behavior is useful when you want to throttle a set of keys independently, for example
/// you may have a web crawler that wants to throttle its requests to each domain independently.
///
/// # Examples
///
/// ```
/// use lib_wc::sync::MultiRateLimiter;
/// use anyhow::Result;
/// use std::time::{Duration, Instant};
/// use std::sync::Arc;
/// use futures::future::join_all;
/// use std::sync::atomic::AtomicUsize;
/// use std::sync::atomic::Ordering::SeqCst;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///    let rate_limiter = Arc::new(MultiRateLimiter::new(Duration::from_millis(50)));
///    static COUNT: AtomicUsize = AtomicUsize::new(0);
///    let start = Instant::now();
///
///    // Spawn 10 tasks, each with a different key
///    join_all(
///       (0..10).map(|key| {
///         let rate_limiter = rate_limiter.clone();
///        tokio::spawn(async move {
///          rate_limiter.throttle(key % 5, || async {
///            COUNT.fetch_add(1, SeqCst);
///          }).await;
///       })
///    })).await;
///
///    // The rate limiter should have throttled the first 5 keys to 1 per 50ms
///    assert!(start.elapsed().as_millis() >= 49);
///
///    // All the keys should have been processed
///    assert_eq!(COUNT.load(SeqCst), 10);
///    Ok(())
/// }
pub struct MultiRateLimiter<K> {
    /// The period for each [`RateLimiter`] associated with a particular key
    period: Duration,

    /// The key-specific [`RateLimiter`]s
    ///
    /// The [`RateLimiter`]s are stored in a [`dashmap::DashMap`], which is a concurrent hash map.
    /// Note that keys may map to the same shard within the [`dashmap::DashMap`], so you may experience
    /// increase latency due to the spin-looping nature of [MultiRateLimiter::throttle] combined
    /// with the fallibility of [`dashmap::DashMap::try_entry`].
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

    /// Throttles the execution of a function based on a key.
    /// Throttling is key-specific, so multiple keys can be throttled independently.
    ///
    /// Uses an exponential backoff to wait for [`dashmap::DashMap`] shards to become available.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_wc::sync::MultiRateLimiter;
    /// use anyhow::Result;
    /// use std::sync::Arc;
    ///
    /// async fn do_work() { /* some computation */ }
    ///
    /// async fn throttle_by_key(the_key: u32, limiter: Arc<MultiRateLimiter<u32>>) {
    ///    limiter.throttle(the_key, || do_work()).await
    /// }
    pub async fn throttle<Fut, F, T>(&self, key: K, f: F) -> T
    where
        Fut: std::future::Future<Output = T>,
        F: FnOnce() -> Fut,
    {
        loop {
            let mut backoff = Backoff::new();
            match self.rate_limiters.try_entry(key.clone()) {
                None => backoff.backoff().await,
                Some(entry) => {
                    let rate_limiter = entry.or_insert_with(|| RateLimiter::new(self.period));
                    return rate_limiter.value().throttle(f).await;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use futures::future::join_all;
    use std::ops::Mul;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering::SeqCst;
    use std::sync::Arc;
    use std::time::Instant;

    #[tokio::test]
    async fn sync_throttle_same_key() -> Result<()> {
        let rate_limiter = MultiRateLimiter::new(Duration::from_millis(10));
        let start = Instant::now();
        static COUNT: AtomicUsize = AtomicUsize::new(0);

        for _ in 0..10 {
            rate_limiter
                .throttle("key", || async {
                    COUNT.fetch_add(1, SeqCst);
                })
                .await;
        }

        assert_eq!(COUNT.load(SeqCst), 10);
        assert!(start.elapsed().as_millis() > 89);
        Ok(())
    }

    #[tokio::test]
    async fn sync_throttle_multi_key() -> Result<()> {
        static ONE_THOUSAND_SECONDS: Duration = Duration::from_secs(1000);
        let rate_limiter = Arc::new(MultiRateLimiter::new(ONE_THOUSAND_SECONDS));
        let start = Instant::now();
        static COUNT: AtomicUsize = AtomicUsize::new(0);

        for k in 0..10 {
            rate_limiter
                .throttle(k, || async {
                    COUNT.fetch_add(1, SeqCst);
                })
                .await;
        }

        assert_eq!(COUNT.load(SeqCst), 10);
        assert!(start.elapsed() < ONE_THOUSAND_SECONDS);
        Ok(())
    }

    #[tokio::test]
    async fn async_throttle_same_key() -> Result<()> {
        let rate_limiter = Arc::new(MultiRateLimiter::new(Duration::from_millis(1)));
        let start = Instant::now();
        static COUNT: AtomicUsize = AtomicUsize::new(0);

        join_all((0..100).map(|_| {
            let rate_limiter = rate_limiter.clone();
            tokio::spawn(async move {
                rate_limiter
                    .throttle("key", || async {
                        COUNT.fetch_add(1, SeqCst);
                    })
                    .await;
                Ok::<(), anyhow::Error>(())
            })
        }))
        .await;

        assert_eq!(COUNT.load(SeqCst), 100);
        assert!(start.elapsed().as_millis() > 99);
        Ok(())
    }

    #[tokio::test]
    async fn async_throttle_multi_key_get_once() -> Result<()> {
        static ONE_THOUSAND_SECONDS: Duration = Duration::from_secs(1000);
        let rate_limiter = Arc::new(MultiRateLimiter::new(ONE_THOUSAND_SECONDS));
        let start = Instant::now();
        static COUNT: AtomicUsize = AtomicUsize::new(0);

        join_all((0..1000).map(|x| {
            let rate_limiter = rate_limiter.clone();
            tokio::spawn(async move {
                rate_limiter
                    .throttle(x, || async {
                        COUNT.fetch_add(1, SeqCst);
                    })
                    .await;
                Ok::<(), anyhow::Error>(())
            })
        }))
        .await;

        assert_eq!(COUNT.load(SeqCst), 1000);
        assert!(start.elapsed() < ONE_THOUSAND_SECONDS);
        Ok(())
    }

    #[tokio::test]
    async fn async_throttle_multi_key_get_many_times() -> Result<()> {
        let period = Duration::from_nanos(100);
        let rate_limiter = Arc::new(MultiRateLimiter::new(period));
        let start = Instant::now();
        let (max, radix): (u32, u32) = (1000, 100);
        let min_wait_time = period.mul(max / radix);
        static COUNT: AtomicUsize = AtomicUsize::new(0);

        join_all((0..max).map(|x| {
            let rate_limiter = rate_limiter.clone();
            tokio::spawn(async move {
                let target = x % radix;
                rate_limiter
                    .throttle(target, || async {
                        COUNT.fetch_add(1, SeqCst);
                    })
                    .await;
                Ok::<(), anyhow::Error>(())
            })
        }))
        .await;

        assert_eq!(COUNT.load(SeqCst), max as usize);
        assert!(start.elapsed() > min_wait_time);
        Ok(())
    }
}
