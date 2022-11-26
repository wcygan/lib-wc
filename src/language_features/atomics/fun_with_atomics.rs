#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicUsize};
    use std::sync::atomic::Ordering::Relaxed;
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;
    use crate::concurrent::executors::multi_threaded::ThreadPool;

    #[test]
    fn basic_atomic_bool() {
        let pool = ThreadPool::new(1);

        let val = Arc::new(AtomicBool::new(false));

        pool.execute({
            let val = val.clone();
            move || {
                val.store(true, Relaxed);
            }
        });

        drop(pool);

        assert_eq!(val.load(Relaxed), true);
    }

    #[test]
    fn counter_progress() {
        let num_done = AtomicUsize::new(0);

        thread::scope(|s| {
            // A background thread to process all 100 items.
            s.spawn(|| {
                for i in 0..100 {
                    sleep(Duration::from_millis(2));
                    num_done.store(i + 1, Relaxed);
                }
            });

            // The main thread shows status updates, every second.
            loop {
                let n = num_done.load(Relaxed);
                if n == 100 { break; }
                println!("Working.. {n}/100 done");
                sleep(Duration::from_millis(50));
            }
        });

        println!("Done!");
    }
}