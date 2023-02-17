use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::Result;

/// A shared map that can be cloned and used in multiple threads
#[derive(Clone)]
pub struct BasicSharedMap<K, V> {
    inner: Arc<Mutex<SharedMapInner<K, V>>>,
}

/// The inner struct that holds the map
struct SharedMapInner<K, V> {
    /// The map
    map: HashMap<K, V>,
}

impl<K, V> Default for BasicSharedMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> BasicSharedMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    /// Create a new shared map
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(SharedMapInner {
                map: HashMap::new(),
            })),
        }
    }

    /// Insert a key-value pair into the map
    pub fn insert(&self, key: K, value: V) {
        let mut inner = self.inner.lock().unwrap();
        inner.map.insert(key, value);
    }

    /// Get a value from the map
    pub fn get(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        let inner = self.inner.lock().unwrap();
        inner.map.get(key).cloned()
    }

    /// Atomically execute a function with a locked, mutable reference to the map
    pub fn with_map<F, R>(&self, func: F) -> Result<R>
    where
        F: FnOnce(&mut HashMap<K, V>) -> R,
    {
        match self.inner.lock() {
            Ok(mut inner) => Ok(func(&mut inner.map)),
            Err(_) => Err(anyhow::anyhow!("Failed to lock mutex")),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use tokio::spawn;
    use tokio::time::sleep;

    use super::*;

    #[test]
    fn test_shared_map() {
        let map = BasicSharedMap::new();
        map.insert("foo", 42);
        assert_eq!(map.get(&"foo"), Some(42));
    }

    #[test]
    fn test_shared_map_clone() {
        let map = BasicSharedMap::new();
        map.insert("foo", 42);
        let map2 = map.clone();
        assert_eq!(map2.get(&"foo"), Some(42));
    }

    #[test]
    fn test_shared_map_clone2() {
        let map = BasicSharedMap::new();
        map.insert("foo", 42);
        let map2 = map.clone();
        map2.insert("bar", 43);
        assert_eq!(map.get(&"bar"), Some(43));
    }

    #[test]
    fn test_shared_map_clone3() {
        let map = BasicSharedMap::new();
        map.insert("foo", 42);
        let map2 = map.clone();
        map2.insert("bar", 43);
        assert_eq!(map.get(&"foo"), Some(42));
    }

    #[test]
    fn test_shared_map_clone4() {
        let map = BasicSharedMap::new();
        map.insert("foo", 42);
        let map2 = map.clone();
        map2.insert("bar", 43);
        let map3 = map2.clone();
        assert_eq!(map3.get(&"foo"), Some(42));
    }

    #[test]
    fn test_with_map() {
        let map = BasicSharedMap::new();
        map.insert("foo", 42);
        let r = map.with_map(|map| {
            assert_eq!(map.get(&"foo"), Some(&42));
        });
        assert!(r.is_ok());
    }

    #[test]
    fn test_with_map2() {
        let map = BasicSharedMap::new();
        map.insert("foo", 42);
        let r = map.with_map(|map| {
            map.insert("bar", 43);
        });
        assert!(r.is_ok());
        assert_eq!(map.get(&"bar"), Some(43));
    }

    #[test]
    fn test_with_map_multiple_threads() {
        let map = BasicSharedMap::new();

        thread::scope(|s| {
            for _ in 0..2 {
                let map = map.clone();
                s.spawn(move || {
                    map.with_map(|map| {
                        let value = map.entry("foo").or_insert(0);
                        if *value == 0 {
                            *value += 1;
                        }
                    })
                    .unwrap();
                });
            }
        });

        assert_eq!(map.get(&"foo"), Some(1))
    }

    #[test]
    fn test_with_map_race() {
        let map = BasicSharedMap::new();

        thread::spawn({
            let map = map.clone();
            move || {
                let _ = map.with_map(|map| {
                    map.insert("a", 1);
                    map.insert("b", 2);
                });
            }
        });

        // Race to see if the writes are visible; both or neither should be visible
        let _ = map.with_map(|map| {
            assert!(
                (map.contains_key("a") && map.contains_key("b"))
                    || (!map.contains_key("a") && !map.contains_key("b"))
            );
        });
    }

    #[tokio::test]
    async fn test_shared_map_with_map_asynchronous_execution() {
        let map = BasicSharedMap::new();

        let count = 100;

        let futures = (0..count).map(|_| {
            let map = map.clone();
            spawn(async move {
                let _ = map.with_map(|map| {
                    let value = map.entry("foo").or_insert(0);
                    *value += 1;
                });
                sleep(tokio::time::Duration::from_nanos(1)).await;
            })
        });

        for future in futures {
            let _ = future.await;
        }

        assert_eq!(map.get(&"foo"), Some(count))
    }
}
