use std::sync::atomic::AtomicUsize;

use lib_wc::executors::{RayonThreadPool, ThreadPool, ThreadPoolError};
use std::sync::atomic::Ordering::Acquire;

fn main() -> Result<(), ThreadPoolError> {
    static COUNT: AtomicUsize = AtomicUsize::new(0);

    let pool = RayonThreadPool::new(2)?;

    pool.spawn(|| println!("All jobs should run before the program exits!"));

    for _ in 0..100 {
        pool.spawn(move || {
            COUNT.fetch_add(1, Acquire);
        });
    }

    pool.shutdown();

    assert_eq!(COUNT.load(Acquire), 100);

    Ok(())
}
