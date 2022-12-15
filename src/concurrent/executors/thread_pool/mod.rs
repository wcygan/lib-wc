pub use basic::BasicThreadPool;
pub use rayon_pool::RayonThreadPool;
use std::io::Error;
use std::thread;

mod basic;
mod rayon_pool;

#[derive(Debug)]
pub enum ThreadPoolError {
    Message(String),
    IOError(std::io::Error),
}

pub type Result<T> = std::result::Result<T, ThreadPoolError>;

/// The trait that all thread pools should implement.
pub trait ThreadPool {
    /// Creates a new thread pool, immediately spawning the specified number of
    /// threads.
    ///
    /// Returns an error if any thread fails to spawn. All previously-spawned threads
    /// are terminated.
    fn new(threads: usize) -> Result<Self>
    where
        Self: Sized;

    /// Spawns a function into the thread pool.
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;

    /// Shuts down the thread pool, waiting for all threads to finish.
    fn shutdown(self);
}

pub fn available_parallelism() -> usize {
    match thread::available_parallelism() {
        Ok(n) => n.get(),
        Err(_) => 1,
    }
}

impl From<std::io::Error> for ThreadPoolError {
    fn from(e: Error) -> Self {
        ThreadPoolError::IOError(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concurrent::executors::thread_pool::rayon_pool::RayonThreadPool;
    use crossbeam::sync::WaitGroup;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    fn spawn_counter<P: ThreadPool>(pool: P) -> Result<()> {
        const TASK_NUM: usize = 20;
        const ADD_COUNT: usize = 1000;

        let wg = WaitGroup::new();
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..TASK_NUM {
            let counter = Arc::clone(&counter);
            let wg = wg.clone();
            pool.spawn(move || {
                for _ in 0..ADD_COUNT {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
                drop(wg);
            })
        }

        wg.wait();
        assert_eq!(counter.load(Ordering::SeqCst), TASK_NUM * ADD_COUNT);
        Ok(())
    }

    #[test]
    fn basic_thread_pool_spawn_counter() -> Result<()> {
        let pool = BasicThreadPool::new(4)?;
        spawn_counter(pool)
    }

    #[test]
    fn rayon_thread_pool_spawn_counter() -> Result<()> {
        let pool = RayonThreadPool::new(4)?;
        spawn_counter(pool)
    }
}
