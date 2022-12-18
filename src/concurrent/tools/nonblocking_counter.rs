use std::sync::atomic::{AtomicUsize, Ordering};

pub struct NonblockingCounter {
    count: AtomicUsize,
}

impl NonblockingCounter {
    pub fn new() -> Self {
        Self {
            count: AtomicUsize::new(0),
        }
    }

    pub fn get(&self, ord: Ordering) -> usize {
        self.count.load(ord)
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
    use crate::concurrent::tools::nonblocking_counter::NonblockingCounter;
    use std::sync::Arc;
    use std::thread::scope;

    #[test]
    fn values_are_in_range() {
        let counter = Arc::new(NonblockingCounter::new());
        let range_max = 20;

        scope(|s| {
            for _ in 0..range_max {
                let counter = counter.clone();
                s.spawn(move || {
                    let res = counter.get_and_increment();
                    assert_eq!(true, res.is_ok());
                    let val = res.unwrap() as i32;
                    let range = 0..range_max;
                    assert_eq!(true, range.contains(&val))
                });
            }
        });

        assert_eq!(range_max as usize, counter.get_and_increment().unwrap());
    }
}
