#[cfg(test)]
mod tests {
    use crate::concurrent::executors::multi_threaded::ThreadPool;
    use std::collections::HashSet;
    use std::sync::atomic::Ordering::Relaxed;
    use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
    use std::sync::{Arc, Once};
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;

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
                if n == 100 {
                    break;
                }
                println!("Working.. {n}/100 done");
                sleep(Duration::from_millis(50));
            }
        });

        println!("Done!");
    }

    #[test]
    fn test_once() {
        static VAL: AtomicUsize = AtomicUsize::new(0);
        static CELL: Once = Once::new();

        for _ in 0..10 {
            CELL.call_once(|| {
                let current = VAL.load(Ordering::SeqCst);
                VAL.compare_exchange(current, current + 1, Ordering::SeqCst, Ordering::SeqCst)
                    .unwrap();
            });
        }

        assert_eq!(1, VAL.load(Ordering::SeqCst));
    }

    #[test]
    fn test_fetch_add() {
        fn allocate_new_id() -> u32 {
            static NEXT_ID: AtomicU32 = AtomicU32::new(0);
            NEXT_ID.fetch_add(1, Relaxed)
        }

        let mut seen: HashSet<u32> = HashSet::new();

        for _ in 0..50 {
            let next = allocate_new_id();
            assert!(!seen.contains(&next));
            seen.insert(next);
        }
    }
}
