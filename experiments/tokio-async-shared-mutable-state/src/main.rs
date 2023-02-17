use lib_wc::sync::ds::BasicSharedMap;
use tokio::{
    task::spawn,
    time::{sleep, Duration},
};

/// This example demonstrates how to use shared mutable state in an asynchronous context
///
/// The `SharedMap` is a wrapper around a `HashMap` that allows for concurrent access & hides the implementation details of the lock
///
/// This follows the rule of "no `.await` while holding a lock"
///
/// See https://draft.ryhl.io/blog/shared-mutable-state/ for more details
#[tokio::main]
async fn main() {
    let map = BasicSharedMap::new();
    let count = 10_000;

    let futures = (0..count).map(|_| {
        let map = map.clone();
        spawn(async move {
            // Temporarily take ownership of the lock & modify the map
            let _ = map.with_map(|map| {
                let value = map.entry("foo").or_insert(0);
                *value += 1;
            });

            // Since we no longer hold the lock, it's okay to `.await` here
            sleep(Duration::from_nanos(1)).await;
        })
    });

    // Wait for all the futures to complete
    for (i, j) in futures.enumerate() {
        j.await.unwrap();
        println!("{} is done", i)
    }

    assert_eq!(map.get(&"foo"), Some(count))
}
