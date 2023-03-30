//! Synchronization tools for concurrent programming

mod backoff;
pub mod ds;

cfg_dangerous! {
    pub use channels::{mpmc, oneshot};
    pub use condvar::Condvar;
    pub use mutex::Mutex;
    pub use naive_mutex::NaiveMutex;
    pub use rw_lock::RwLock;
    pub use semaphore::Semaphore;
    pub use spinlock::SpinLock;
    mod channels;
    mod condvar;
    mod mutex;
    mod naive_mutex;
    mod rw_lock;
    mod semaphore;
    mod spinlock;
}
