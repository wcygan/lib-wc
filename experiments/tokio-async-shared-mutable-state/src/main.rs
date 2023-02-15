use lib_wc::concurrent::data_structures::maps::simple_shared_map::SharedMap;
use tokio::{
    task::spawn,
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() {
    let map = SharedMap::new();

    let count = 10_000;

    let futures = (0..count).map(|_| {
        let map = map.clone();
        spawn(async move {
            let _ = map.with_map(|map| {
                // Enter the critical section 
                let value = map.entry("foo").or_insert(0);
                *value += 1;
                // Exit the critical section
            });

            // Since we no longer hold the lock, it's okay to `.await` here
            sleep(Duration::from_nanos(1)).await;
        })
    });

    for (i, j) in futures.enumerate() {
        j.await.unwrap();
        println!("{} is done", i)
    }

    assert_eq!(map.get(&"foo"), Some(count))
}
