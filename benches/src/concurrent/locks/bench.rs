use wc::concurrent::locks::{Mutex, NaiveMutex, SpinLock};

static ITERATIONS: usize = 5_000_000;

fn tests(bh: &mut criterion::Criterion) {
    bh.bench_function("naive mutex uncontended", |bh| {
        bh.iter(|| {
            let m = NaiveMutex::new(0);
            for _ in 0..ITERATIONS {
                *m.lock() += 1;
            }
        })
    });

    bh.bench_function("mutex uncontended", |bh| {
        bh.iter(|| {
            let m = Mutex::new(0);
            for _ in 0..ITERATIONS {
                *m.lock() += 1;
            }
        })
    });

    bh.bench_function("spinlock uncontended", |bh| {
        bh.iter(|| {
            let m = SpinLock::new(0);
            for _ in 0..ITERATIONS {
                *m.lock() += 1;
            }
        })
    });
}

criterion_group!(
    name = bench;
    config = crate::default_config();
    targets = tests
);
