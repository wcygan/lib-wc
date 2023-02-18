use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicU32, AtomicUsize};

use atomic_wait::{wait, wake_all, wake_one};

use crate::concurrent::sync::mutex::MutexGuard;

/// a primitive to signal and wait on a condition
pub struct Condvar {
    counter: AtomicU32,
    num_waiters: AtomicUsize,
}

impl Condvar {
    /// Create a new condition variable
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_wc::sync::Condvar;
    ///
    /// let condvar = Condvar::new();
    /// ```
    pub const fn new() -> Self {
        Self {
            counter: AtomicU32::new(0),
            num_waiters: AtomicUsize::new(0),
        }
    }

    /// Wait on the condition variable until notified.
    ///
    /// This function will atomically unlock the mutex, and then wait for a notification.
    ///
    /// # Examples
    ///
    /// ```
    ///  use std::thread;
    ///  use std::time::Duration;
    ///  use std::sync::Arc;
    ///  use lib_wc::sync::{Condvar, Mutex};
    ///
    ///  let mutex = Arc::new(Mutex::new(0));
    ///  let condvar = Condvar::new();    
    ///  let mutex = Mutex::new(0);
    ///  let condvar = Condvar::new();
    ///
    ///  let mut wakeups = 0;
    ///
    ///  thread::scope(|s| {
    ///    s.spawn(|| {
    ///       thread::sleep(Duration::from_nanos(10));
    ///       *mutex.lock() = 123;
    ///       condvar.notify_one();
    ///    });
    ///
    ///    let mut m = mutex.lock();
    ///    while *m < 100 {
    ///      m = condvar.wait(m);
    ///      wakeups += 1;
    ///    }
    ///
    ///    assert_eq!(*m, 123);
    ///  });
    ///
    ///  // Check that the main thread actually did wait (not busy-loop),
    ///  // while still allowing for a few spurious wake ups.
    ///  assert!(wakeups < 10);
    /// ```
    pub fn wait<'a, T>(&self, mutex_guard: MutexGuard<'a, T>) -> MutexGuard<'a, T> {
        self.num_waiters.fetch_add(1, Relaxed);
        let counter_value = self.counter.load(Relaxed);

        // Unlock the mutex by dropping the guard,
        // but remember the mutex so we can lock it again later.
        let mutex = mutex_guard.mutex;
        drop(mutex_guard);

        // Wait, but only if the counter hasn't changed since unlocking.
        wait(&self.counter, counter_value);

        self.num_waiters.fetch_sub(1, Relaxed);

        mutex.lock()
    }

    pub fn notify_one(&self) {
        if self.num_waiters.load(Relaxed) > 0 {
            self.counter.fetch_add(1, Relaxed);
            wake_one(&self.counter)
        }
    }

    pub fn notify_all(&self) {
        if self.num_waiters.load(Relaxed) > 0 {
            self.counter.fetch_add(1, Relaxed);
            wake_all(&self.counter)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use quickcheck_macros::quickcheck;

    use crate::concurrent::sync::Mutex;

    use super::*;

    #[quickcheck]
    fn test_condvar() {
        let mutex = Mutex::new(0);
        let condvar = Condvar::new();

        let mut wakeups = 0;

        thread::scope(|s| {
            s.spawn(|| {
                thread::sleep(Duration::from_nanos(10));
                *mutex.lock() = 123;
                condvar.notify_one();
            });

            let mut m = mutex.lock();
            while *m < 100 {
                m = condvar.wait(m);
                wakeups += 1;
            }

            assert_eq!(*m, 123);
        });

        // Check that the main thread actually did wait (not busy-loop),
        // while still allowing for a few spurious wake ups.
        assert!(wakeups < 10);
    }
}
