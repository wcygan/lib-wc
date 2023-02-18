use super::*;
use crossbeam::sync::WaitGroup;

/// Thin wrapper of rayon::ThreadPool which allows the use of [`ThreadPool::shutdown`] methpd
pub struct RayonThreadPool {
    inner: rayon::ThreadPool,
    wg: WaitGroup,
}

impl ThreadPool for RayonThreadPool {
    /// Create a new thread pool with the given number of threads
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_wc::executors::{RayonThreadPool, ThreadPool};
    /// let tp = RayonThreadPool::new(4).unwrap();
    /// ```
    fn new(threads: usize) -> Result<Self> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .map_err(|e| ThreadPoolError::Message(format!("{e}")))?;
        Ok(RayonThreadPool {
            inner: pool,
            wg: WaitGroup::new(),
        })
    }

    /// Spawn a new task on the thread pool
    ///
    /// # Examples
    ///
    /// ```
    ///   use lib_wc::executors::{RayonThreadPool, ThreadPool};
    ///
    ///   let tp = RayonThreadPool::new(4).unwrap();
    ///
    ///   tp.spawn(|| {
    ///     println!("Hello from a thread!");
    ///   });
    /// ```
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // wg is used to signal that pending work has been finished
        let wg = self.wg.clone();
        self.inner.spawn(move || {
            job();
            drop(wg);
        });
    }

    /// Wait for all currently running tasks to complete
    ///
    /// # Examples
    ///
    /// ```
    ///   use lib_wc::executors::{RayonThreadPool, ThreadPool};    
    ///   use std::sync::atomic::{AtomicUsize, Ordering};
    ///
    ///   static ATOMIC_COUNTER: AtomicUsize = AtomicUsize::new(0);
    ///
    ///   let tp = RayonThreadPool::new(4).unwrap();
    ///
    ///   for _ in 0..100 {
    ///     tp.spawn(|| {
    ///       ATOMIC_COUNTER.fetch_add(1, Ordering::Acquire);
    ///     });
    ///   }
    ///
    ///   tp.shutdown();
    ///
    ///   assert_eq!(ATOMIC_COUNTER.load(Ordering::Relaxed), 100);
    ///
    /// ```
    fn shutdown(self) {
        self.wg.wait();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_rayon_thread_pool() {
        let pool = RayonThreadPool::new(4).unwrap();
        let counter = Arc::new(AtomicUsize::new(0));
        for _ in 0..100 {
            let counter = counter.clone();
            pool.spawn(move || {
                counter.fetch_add(1, Ordering::Acquire);
            });
        }
        pool.shutdown();
        assert_eq!(counter.load(Ordering::Relaxed), 100);
    }

    #[test]
    fn test_wait_no_jobs() {
        let pool = RayonThreadPool::new(4).unwrap();
        pool.shutdown();
    }
}
