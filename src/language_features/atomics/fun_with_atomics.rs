#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use crate::concurrent::executors::multi_threaded::ThreadPool;

    #[test]
    fn basic_atomic_bool() {
        let pool = ThreadPool::new(1);

        let val = Arc::new(AtomicBool::new(false));

        pool.execute({
            let val = val.clone();
            move || {
                val.store(true, std::sync::atomic::Ordering::Relaxed);
            }
        });

        drop(pool);

        assert_eq!(val.load(std::sync::atomic::Ordering::Relaxed), true);
    }
}