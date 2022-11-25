use std::sync::atomic::{AtomicUsize, Ordering};

struct NonblockingCounter {
    count: AtomicUsize,
}

impl NonblockingCounter {
    pub fn new() -> Self {
        Self {
            count: AtomicUsize::new(0),
        }
    }

    /// Get the current value of the counter and increment it
    pub fn get_and_increment(&self) -> Result<usize, usize> {
        let mut current_count = self.count.load(Ordering::SeqCst);

        loop {
            match self.count.compare_exchange_weak(
                current_count,
                current_count + 1,
                Ordering::Acquire,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(count) => {
                    current_count = count;
                }
            }
        }

        Ok(current_count)
    }
}

impl Default for NonblockingCounter {
    fn default() -> Self {
        NonblockingCounter::new()
    }
}

unsafe impl Send for NonblockingCounter {}
unsafe impl Sync for NonblockingCounter {}

#[cfg(test)]
mod tests {
    use crate::concurrent::executors::multi_threaded::ThreadPool;
    use crate::concurrent::tools::nonblocking_counter::NonblockingCounter;
    use std::sync::Arc;

    #[test]
    fn values_are_in_range() {
        let counter = Arc::new(NonblockingCounter::new());

        // spawn a thread pool
        let mut pool = ThreadPool::new(8);

        let range_max = 20;
        // spawn the tasks
        for i in 0..range_max {
            let counter_clone = counter.clone();
            pool.execute(move || {
                let res = counter_clone.get_and_increment();
                assert_eq!(true, res.is_ok());
                let val = res.unwrap() as i32;
                let range = 0..range_max;
                assert_eq!(true, range.contains(&val))
            });
        }

        // wait for the pool to finish all of the tasks
        drop(pool);

        // verify that the counter is
        assert_eq!(range_max as usize, counter.get_and_increment().unwrap());
    }
}
