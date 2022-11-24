use std::sync::atomic::{AtomicUsize, Ordering};

struct NonblockingCounter {
    count: AtomicUsize,
}

impl NonblockingCounter {
    pub fn new() -> Self {
        Self { count: AtomicUsize::new(0) }
    }

    pub fn get_and_increment(&self) -> Result<usize, usize> {
        let current_value = self.count.load(Ordering::SeqCst);

        loop {
            let res = self.count.compare_exchange(
                current_value,
                current_value + 1,
                Ordering::SeqCst,
                Ordering::SeqCst,
            );

            if res.is_ok() {
                break;
            } else {
                return res
            }
        }

        Ok(current_value)
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
    use std::sync::Arc;
    use crate::concurrent::executors::multi_threaded::ThreadPool;
    use crate::concurrent::tools::nonblocking_counter::NonblockingCounter;

    #[test]
    fn values_are_in_range() {
        let counter = Arc::new(NonblockingCounter::new());

        // spawn a thread pool
        let mut pool = ThreadPool::new(8);

        // spawn the tasks
        for i in 0..20 {
            let counter_clone = counter.clone();
            pool.execute(move || {
                let res = counter_clone.get_and_increment();
                assert_eq!(true, res.is_ok());
                let val = res.unwrap() as i32;
                let range = 0..20;
                assert_eq!(true, range.contains(&val))
            });
        }

        // wait for the pool to finish all of the tasks
        drop(pool);
    }
}