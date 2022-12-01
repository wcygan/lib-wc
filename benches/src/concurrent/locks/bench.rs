use wc::concurrent::locks::{Mutex, NaiveMutex, SpinLock};

static ITERATIONS: usize = 100_000;

// This will benchmark a lock under no contention
// This means that the current thread is the only thread attempting to acquire the lock
macro_rules! lock_uncontended(
    ($name: ident, $lockType: ty) => {
        fn $name(bh: &mut criterion::Criterion) {
            bh.bench_function(stringify!($name), move |bh| bh.iter(|| {
                let lock = <$lockType>::new(0);
                for _ in 0..ITERATIONS {
                    *lock.lock() += 1;
                }
            }));
        }
    }
);

// This will benchmark a lock under contention
// This means that multiple threads are attempting to acquire the lock concurrently
macro_rules! lock_with_contention(
    ($name: ident, $lockType: ty) => {
        fn $name(bh: &mut criterion::Criterion) {
            bh.bench_function(stringify!($name), move |bh| bh.iter(|| {
                let lock = <$lockType>::new(0);
                std::thread::scope(|s| {
                    for _ in 0..16 {
                        s.spawn(|| {
                            for _ in 0..ITERATIONS {
                                *lock.lock() += 1;
                            }
                        });
                    }
                });
            }));
        }
    }
);

lock_uncontended!(mutex_uncontended, Mutex<u32>);
lock_uncontended!(naive_mutex_uncontended, NaiveMutex<u32>);
lock_uncontended!(spinlock_uncontended, SpinLock<u32>);
lock_with_contention!(mutex_with_contention, Mutex<u32>);
lock_with_contention!(naive_mutex_with_contention, NaiveMutex<u32>);
lock_with_contention!(spinlock_with_contention, SpinLock<u32>);

criterion_group!(
    name = bench;
    config = crate::default_config();
    targets = mutex_uncontended, naive_mutex_uncontended, spinlock_uncontended, mutex_with_contention, naive_mutex_with_contention, spinlock_with_contention
);
