use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;

/// This function will return the same ID for the lifetime of the program..
/// Different executions of the program will generate different IDs.
fn global_id() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    let key = KEY.load(Relaxed);
    if key == 0 {
        let new_key = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            % 1000000;
        let new_key = (new_key as u64) + 1; // key can never be 0
        match KEY.compare_exchange(0, new_key, Relaxed, Relaxed) {
            Ok(_) => new_key,
            Err(k) => k,
        }
    } else {
        key
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concurrent::executors::multi_threaded::ThreadPool;
    use std::collections::HashSet;
    use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
    use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};
    use std::sync::{Arc, Once};
    use std::thread;
    use std::thread::{sleep, Thread};
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

    #[test]
    fn test_compare_exchange_key_set_in_main() {
        let key = Arc::new(global_id());
        let pool = ThreadPool::default();

        for _ in 0..10 {
            let key = key.clone();
            pool.execute(move || {
                // test: all threads get the same key
                let new_key = global_id();
                assert_eq!(*key, new_key);
            });
        }
    }

    #[test]
    fn test_compare_exchange_key_set_in_threads() {
        let (sender, receiver) = std::sync::mpsc::channel::<u64>();
        let pool = ThreadPool::default();

        for _ in 0..10 {
            let sender = sender.clone();
            pool.execute(move || {
                sender.send(global_id()).unwrap();
            });
        }

        let mut values: Vec<u64> = vec![];
        for _ in 0..10 {
            values.push(receiver.recv().unwrap());
        }

        // test: all values are the same
        let first = values[0];
        for v in values {
            assert_eq!(first, v);
        }
    }

    #[test]
    fn test_acquire_release() {
        static DATA: AtomicU64 = AtomicU64::new(0);
        static READY: AtomicBool = AtomicBool::new(false);

        thread::spawn(|| {
            DATA.store(123, Relaxed);
            READY.store(true, Release); // Everything before this store ..
        });

        while !READY.load(Acquire) {
            // .. is visible after this load, if it loads `true`.
            sleep(Duration::from_millis(100));
            println!("waiting...");
        }

        assert_eq!(123, DATA.load(Relaxed))
    }

    #[test]
    fn test_acquire_release_with_mutex() {
        // this example uses LOCKED as a mutex for DATA. Threads attempted to lock DATA and do some computation with it
        static mut DATA: String = String::new();
        static LOCKED: AtomicBool = AtomicBool::new(false);

        fn f() {
            if LOCKED
                .compare_exchange(false, true, Acquire, Relaxed)
                .is_ok()
            {
                // Safety: We hold the exclusive lock, so nothing else is accessing DATA.
                unsafe { DATA.push('!') };
                LOCKED.store(false, Release);
            }
        }

        thread::scope(|s| {
            for _ in 0..100 {
                s.spawn(f);
            }
        });

        unsafe {
            println!("{}", DATA);
            println!("{}", DATA.len());
        }
    }
}
