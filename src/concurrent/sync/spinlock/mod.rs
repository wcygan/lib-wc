use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::*;

/// a primitive for mutual exclusion that spins in a loop
pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> SpinLock<T> {
    /// Creates a new spinlock.
    ///
    /// # Examples
    ///
    /// ```
    ///   use lib_wc::sync::SpinLock;
    ///
    ///   let spinlock = SpinLock::new(0);
    ///
    ///   {
    ///     let mut guard = spinlock.lock();
    ///     *guard += 1;
    ///   } // The guard is dropped here, unlocking the mutex
    ///
    ///   {
    ///     let mut guard = spinlock.lock();
    ///     *guard += 1;
    ///   } // The guard is dropped here, unlocking the mutex
    ///
    ///   assert_eq!(*spinlock.lock(), 2);
    /// ```
    pub fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    /// Acquires a lock on the spinlock, spinning the current thread in a loop until it is able to do so.
    ///
    /// This function returns a `Guard` which will release the lock when dropped.
    ///
    /// # Examples
    /// ```
    ///   use lib_wc::sync::SpinLock;
    ///
    ///   let spinlock = SpinLock::new(0);
    ///
    ///   {
    ///     let mut guard = spinlock.lock();
    ///     *guard += 1;
    ///   } // The guard is dropped here, unlocking the mutex
    ///
    ///   {
    ///     let mut guard = spinlock.lock();
    ///     *guard += 1;
    ///   } // The guard is dropped here, unlocking the mutex
    ///
    ///   assert_eq!(*spinlock.lock(), 2);
    /// ```
    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        Guard { lock: self }
    }
}

// T doesn't need to be sync because only one thread will have access to it at a time
unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety: the existence of this guard guarantees
        // that we have exclusively locked the lock
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: the existence of this guard guarantees
        // that we have exclusively locked the lock
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
    }
}

#[cfg(test)]
mod tests {
    use crate::concurrent::sync::SpinLock;

    #[test]
    fn test_spinlock() {
        let x = SpinLock::new(Vec::new());

        std::thread::scope(|s| {
            s.spawn(|| x.lock().push(1));
            s.spawn(|| {
                let mut g = x.lock();
                g.push(2);
                g.push(2);
            });
        });
        let g = x.lock();
        assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]);
    }

    #[test]
    fn test_multiple_threads() {
        let x = SpinLock::new(Vec::new());

        std::thread::scope(|s| {
            s.spawn(|| x.lock().push(1));

            for _ in 0..100 {
                s.spawn(|| x.lock().push(1));
            }
        });

        assert_eq!(x.lock().len(), 101);
    }
}
