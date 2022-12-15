pub use basic::BasicThreadPool;
use std::io::Error;
mod basic;

mod shared_queue;

#[derive(Debug)]
pub enum ThreadPoolError {
    IOError(std::io::Error),
}

pub type Result<T> = std::result::Result<T, ThreadPoolError>;

/// The trait that all thread pools should implement.
pub trait ThreadPool: Default {
    /// Creates a new thread pool, immediately spawning the specified number of
    /// threads.
    ///
    /// Returns an error if any thread fails to spawn. All previously-spawned threads
    /// are terminated.
    fn new(threads: usize) -> Result<Self>
    where
        Self: Sized;

    /// Spawns a function into the thread pool.
    ///
    /// Spawning always succeeds, but if the function panics the threadpool continues
    /// to operate with the same number of threads &mdash; the thread count is not
    /// reduced nor is the thread pool destroyed, corrupted or invalidated.
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}

impl From<std::io::Error> for ThreadPoolError {
    fn from(e: Error) -> Self {
        ThreadPoolError::IOError(e)
    }
}
