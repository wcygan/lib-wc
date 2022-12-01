use atomic_wait::{wait, wake_one};
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::{Acquire, Release};

static UNLOCKED: u32 = 0;
static LOCKED: u32 = 1;

/// This mutex is naive because it optimize the scenario where multiple threads are in
/// contention for the lock. See [crate::concurrent::locks::Mutex] for an optimized implementation.
pub struct NaiveMutex<T> {
    state: AtomicU32,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for NaiveMutex<T> where T: Send {}

pub struct MutexGuard<'a, T> {
    mutex: &'a NaiveMutex<T>,
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        self.mutex.state.store(UNLOCKED, Release);
        wake_one(&self.mutex.state);
    }
}

impl<T> NaiveMutex<T> {
    pub fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(UNLOCKED),
            value: UnsafeCell::new(value),
        }
    }

    #[inline]
    pub fn lock(&self) -> MutexGuard<T> {
        while self.state.swap(LOCKED, Acquire) == LOCKED {
            wait(&self.state, LOCKED);
        }
        MutexGuard { mutex: self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread::scope;

    #[test]
    fn test_mutex() {
        let mutex = Arc::new(NaiveMutex::new(0));

        scope(|s| {
            for _ in 0..10 {
                let mutex = mutex.clone();
                s.spawn(move || {
                    for _ in 0..1000 {
                        let mut guard = mutex.lock();
                        *guard += 1;
                    }
                });
            }
        });

        assert_eq!(mutex.lock().deref(), &10000);
    }
}
