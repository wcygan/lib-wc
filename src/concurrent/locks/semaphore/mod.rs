use atomic_wait::{wait, wake_one};
use std::cell::UnsafeCell;
use std::ops::Deref;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release, SeqCst};

pub struct Semaphore<T> {
    usage: AtomicU32,
    capacity: u32,
    resource: UnsafeCell<T>,
}

pub struct SemaphoreGuard<'a, T> {
    semaphore: &'a Semaphore<T>,
}

impl<T> Semaphore<T> {
    pub fn new(resource: T, capacity: u32) -> Self {
        Self {
            usage: AtomicU32::new(0),
            capacity,
            resource: UnsafeCell::new(resource),
        }
    }

    pub fn acquire(&self) -> SemaphoreGuard<T> {
        let mut count = self.usage.load(Relaxed);

        loop {
            match count < self.capacity {
                true => {
                    match self
                        .usage
                        .compare_exchange(count, count + 1, Acquire, Relaxed)
                    {
                        Ok(_) => return SemaphoreGuard { semaphore: self },
                        Err(e) => count = e,
                    }
                }
                false => {
                    wait(&self.usage, count);
                    count = self.usage.load(Relaxed);
                }
            }
        }
    }
}

unsafe impl<T> Sync for Semaphore<T> where T: Send + Sync {}

impl<T> SemaphoreGuard<'_, T> {
    fn usage(&self) -> u32 {
        self.semaphore.usage.load(SeqCst)
    }
}

impl<T> Deref for SemaphoreGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.semaphore.resource.get() }
    }
}

impl<T> Drop for SemaphoreGuard<'_, T> {
    fn drop(&mut self) {
        if self.semaphore.usage.fetch_sub(1, Release) == self.semaphore.capacity {
            wake_one(&self.semaphore.usage);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread::scope;

    #[test]
    fn test_semaphore_capacity() {
        let capacity = 10;
        let thread_count = 25;
        let semaphore = Arc::new(Semaphore::new((), capacity));
        let barrier = Arc::new(AtomicU32::new(0));
        let (sender, receiver) = std::sync::mpsc::channel::<u32>();

        scope(|s| {
            for _ in 0..thread_count {
                let sender = sender.clone();
                let barrier = barrier.clone();
                let semaphore = semaphore.clone();

                s.spawn(move || {
                    barrier.fetch_add(1, Acquire);
                    let guard = semaphore.acquire();
                    // Wait until the barrier is removed
                    while barrier.load(Relaxed) != u32::MAX {}
                    // Check how many threads are using the barrier
                    let val = guard.usage();
                    sender.send(val).unwrap();
                });
            }

            s.spawn(move || {
                // Wait for threads to start
                while barrier.load(Relaxed) != thread_count {}
                // Release all threads
                barrier.store(u32::MAX, Release);
            });
        });

        let mut results = Vec::new();
        for _ in 0..thread_count {
            results.push(receiver.recv().unwrap());
        }

        // At least one of the threads should see a usage value equal to
        // the capacity of the semaphore
        let max_value = results.iter().max().unwrap();
        assert_eq!(max_value, &capacity);
    }
}
