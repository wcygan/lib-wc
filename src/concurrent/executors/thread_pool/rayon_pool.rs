use super::*;
use crossbeam::sync::WaitGroup;

/// Thin wrapper of rayon::ThreadPool which allows the use of [`ThreadPool::shutdown`] methpd
pub struct RayonThreadPool {
    inner: rayon::ThreadPool,
    wg: WaitGroup,
}

impl ThreadPool for RayonThreadPool {
    fn new(threads: usize) -> Result<Self> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .map_err(|e| ThreadPoolError::Message(format!("{}", e)))?;
        Ok(RayonThreadPool {
            inner: pool,
            wg: WaitGroup::new(),
        })
    }

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
