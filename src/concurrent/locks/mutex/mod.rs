use atomic_wait::{wait, wake_one};
use std::cell::UnsafeCell;
use std::hint::spin_loop;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

static UNLOCKED: u32 = 0;
static LOCKED: u32 = 1;
static LOCKED_WITH_WAITERS: u32 = 2;

pub struct Mutex<T> {
    state: AtomicU32,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
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
        if self.mutex.state.swap(UNLOCKED, Release) == LOCKED_WITH_WAITERS {
            wake_one(&self.mutex.state);
        }
    }
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(UNLOCKED),
            value: UnsafeCell::new(value),
        }
    }

    #[inline]
    pub fn lock(&self) -> MutexGuard<T> {
        if self
            .state
            .compare_exchange(UNLOCKED, LOCKED, Acquire, Relaxed)
            .is_err()
        {
            lock_contended(&self.state)
        }
        MutexGuard { mutex: self }
    }
}

#[cold]
fn lock_contended(state: &AtomicU32) {
    let mut spin_count = 0;

    while state.load(Relaxed) == LOCKED && spin_count < 100 {
        spin_count += 1;
        spin_loop()
    }

    if state
        .compare_exchange(UNLOCKED, LOCKED, Acquire, Relaxed)
        .is_ok()
    {
        return;
    }

    while state.swap(LOCKED_WITH_WAITERS, Acquire) != UNLOCKED {
        wait(state, LOCKED_WITH_WAITERS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread::scope;

    #[test]
    fn test_mutex() {
        let mutex = Arc::new(Mutex::new(0));

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