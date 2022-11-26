#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::iter::once;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Condvar, Mutex, Once};
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn spawn_a_thread() {
        let t = thread::spawn(|| {
            println!("Hello from a thread!");
        });

        t.join().unwrap()
    }

    #[test]
    fn scoped_threads() {
        // thread::scope spawns "scoped" threads that cannot outlive the scope of the
        // closures that we pass to it.
        // scopes allow us to safely borrow data from the parent thread
        let numbers = vec![1, 2, 3];

        thread::scope(|s| {
            s.spawn(|| {
                println!("length: {}", numbers.len());
            });

            s.spawn(|| {
                for n in &numbers {
                    println!("{n}");
                }
            });
        });
    }

    #[test]
    fn arc_shadowing() {
        let x = Arc::new(5);

        let t = thread::spawn({
            // create a value "x" in a new scope
            // the value "x" shadows the value "x" in the parent scope
            let x = x.clone();
            move || {
                println!("{}", x);
            }
        });

        t.join().unwrap();

        println!("{}", x);
    }

    #[test]
    fn thread_parking() {
        let counter = Arc::new(Mutex::new(0));

        thread::scope({
            let counter = counter.clone();
            |s| {
                let t = s.spawn({
                    move || {
                        let mut counter = counter.lock().unwrap();

                        // park until someone else wakes us up
                        thread::park();
                        *counter += 1;
                    }
                });

                // wake up the parked thread
                t.thread().unpark();
            }
        });

        let value = *counter.lock().unwrap();
        assert_eq!(1, value)
    }

    #[test]
    fn wait_notify_with_two_threads() {
        let counter = Arc::new(Mutex::new(0));
        let condvar = Arc::new(Condvar::new());

        let waiter = thread::spawn({
            let counter = counter.clone();
            let condvar = condvar.clone();
            move || {
                let mut counter = counter.lock().unwrap();

                while *counter == 0 {
                    counter = condvar.wait(counter).unwrap();
                }

                *counter += 1;
            }
        });

        let signaler = thread::spawn({
            let counter = counter.clone();
            let condvar = condvar.clone();
            move || {
                let mut counter = counter.lock().unwrap();

                // signal the waiter to wake up
                *counter += 1;
                condvar.notify_one();
            }
        });

        signaler.join().unwrap();
        waiter.join().unwrap();

        let value = *counter.lock().unwrap();
        assert_eq!(2, value)
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
}
