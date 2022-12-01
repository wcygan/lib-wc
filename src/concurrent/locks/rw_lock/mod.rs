use atomic_wait::{wait, wake_all, wake_one};
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

pub struct RwLock<T> {
    /// The number of read locks times two, plus one if there's a writer waiting.
    /// u32::MAX for write locked.
    ///
    /// This means that readers may acquire the lock when
    /// the state is even, but need to block when odd.
    state: AtomicU32,
    /// Incremented to wake up writers.
    writer_wake_counter: AtomicU32,
    value: UnsafeCell<T>,
}

pub struct ReadGuard<'a, T> {
    rwlock: &'a RwLock<T>,
}

pub struct WriteGuard<'a, T> {
    rwlock: &'a RwLock<T>,
}

unsafe impl<T> Sync for RwLock<T> where T: Send + Sync {}

impl<T> RwLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            writer_wake_counter: AtomicU32::new(0),
            value: UnsafeCell::new(value),
        }
    }

    pub fn read(&self) -> ReadGuard<T> {
        let mut s = self.state.load(Relaxed);

        loop {
            match s % 2 == 0 {
                true => {
                    assert_ne!(s, u32::MAX - 2, "too many readers");
                    match self.state.compare_exchange_weak(s, s + 2, Acquire, Relaxed) {
                        Ok(_) => return ReadGuard { rwlock: self },
                        Err(e) => s = e,
                    }
                }
                false => {
                    wait(&self.state, s);
                    s = self.state.load(Relaxed);
                }
            }
        }
    }

    pub fn write(&self) -> WriteGuard<T> {
        let mut s = self.state.load(Relaxed);
        loop {
            // Try to lock if unlocked.
            if s <= 1 {
                match self.state.compare_exchange(s, u32::MAX, Acquire, Relaxed) {
                    Ok(_) => return WriteGuard { rwlock: self },
                    Err(e) => {
                        s = e;
                        continue;
                    }
                }
            }

            // Block new readers, by making sure the state is odd.
            if s % 2 == 0 {
                match self.state.compare_exchange(s, s + 1, Relaxed, Relaxed) {
                    Ok(_) => {}
                    Err(e) => {
                        s = e;
                        continue;
                    }
                }
            }

            // Wait if it's still locked
            let w = self.writer_wake_counter.load(Acquire);
            s = self.state.load(Relaxed);

            if s >= 2 {
                wait(&self.writer_wake_counter, w);
                s = self.state.load(Relaxed);
            }
        }
    }
}

impl<T> Drop for ReadGuard<'_, T> {
    fn drop(&mut self) {
        // Decrement the state by 2 to remove one read-lock.
        if self.rwlock.state.fetch_sub(2, Release) == 3 {
            // If we decremented from 3 to 1, that means
            // the RwLock is now unlocked _and_ there is
            // a waiting writer, which we wake up.
            self.rwlock.writer_wake_counter.fetch_add(1, Release);
            wake_one(&self.rwlock.writer_wake_counter)
        }
    }
}

impl<T> Drop for WriteGuard<'_, T> {
    fn drop(&mut self) {
        self.rwlock.state.store(0, Release);
        self.rwlock.writer_wake_counter.fetch_add(1, Release);
        wake_one(&self.rwlock.writer_wake_counter);
        wake_all(&self.rwlock.state);
    }
}

impl<T> Deref for WriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.rwlock.value.get() }
    }
}

impl<T> DerefMut for WriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.rwlock.value.get() }
    }
}

impl<T> Deref for ReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.rwlock.value.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hint::spin_loop;
    use std::sync::Arc;
    use std::thread;
    use std::thread::scope;

    #[test]
    fn test_write_read() {
        let rwlock = Arc::new(RwLock::new(0));

        thread::spawn({
            let rwlock = rwlock.clone();
            move || {
                let mut guard = rwlock.write();
                *guard = 1;
            }
        })
        .join()
        .unwrap();

        thread::spawn({
            let rwlock = rwlock.clone();
            move || {
                let guard = rwlock.read();
                assert_eq!(*guard, 1);
            }
        })
        .join()
        .unwrap();
    }

    #[test]
    fn test_read_write_read() {
        let barrier = Arc::new(AtomicU32::new(0));
        let rwlock = Arc::new(RwLock::new(0));

        scope(|s| {
            // the first domino will read 0
            s.spawn(|| {
                let r = rwlock.read();
                assert_eq!(*r, 0);
                barrier.fetch_add(1, Release);
            });

            // the second domino will increment it by one
            s.spawn(|| {
                while barrier.load(Acquire) == 0 {
                    spin_loop();
                }

                let mut w = rwlock.write();
                *w += 1;
                barrier.fetch_add(1, Release);
            });

            // the third domino will read 1
            s.spawn(|| {
                while barrier.load(Acquire) == 1 {
                    spin_loop();
                }

                let r = rwlock.read();
                assert_eq!(*r, 1);
            });
        })
    }
}
