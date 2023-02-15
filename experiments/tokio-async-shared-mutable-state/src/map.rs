use anyhow::{Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A shared map that can be cloned and used in multiple threads
#[derive(Clone)]
pub struct SharedMap<K, V> {
    inner: Arc<Mutex<SharedMapInner<K, V>>>,
}

/// The inner struct that holds the map
struct SharedMapInner<K, V> {
    /// The map
    map: HashMap<K, V>,
}

impl<K, V> SharedMap<K, V>
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

    /// Execute a function with a mutable reference to the map
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
    use super::*;
    use std::thread;
    use tokio::spawn;

    #[test]
    fn test_shared_map() {
        let map = SharedMap::new();
        map.insert("foo", 42);
        assert_eq!(map.get(&"foo"), Some(42));
    }

    #[test]
    fn test_shared_map_clone() {
        let map = SharedMap::new();
        map.insert("foo", 42);
        let map2 = map.clone();
        assert_eq!(map2.get(&"foo"), Some(42));
    }

    #[test]
    fn test_shared_map_clone2() {
        let map = SharedMap::new();
        map.insert("foo", 42);
        let map2 = map.clone();
        map2.insert("bar", 43);
        assert_eq!(map.get(&"bar"), Some(43));
    }

    #[test]
    fn test_shared_map_clone3() {
        let map = SharedMap::new();
        map.insert("foo", 42);
        let map2 = map.clone();
        map2.insert("bar", 43);
        assert_eq!(map.get(&"foo"), Some(42));
    }

    #[test]
    fn test_shared_map_clone4() {
        let map = SharedMap::new();
        map.insert("foo", 42);
        let map2 = map.clone();
        map2.insert("bar", 43);
        let map3 = map2.clone();
        assert_eq!(map3.get(&"foo"), Some(42));
    }

    #[test]
    fn test_with_map() {
        let map = SharedMap::new();
        map.insert("foo", 42);
        let r = map.with_map(|map| {
            assert_eq!(map.get(&"foo"), Some(&42));
        });
        assert!(r.is_ok());
    }

    #[test]
    fn test_with_map2() {
        let map = SharedMap::new();
        map.insert("foo", 42);
        let r = map.with_map(|map| {
            map.insert("bar", 43);
        });
        assert!(r.is_ok());
        assert_eq!(map.get(&"bar"), Some(43));
    }

    #[test]
    fn test_with_map_multiple_threads() {
        let map = SharedMap::new();

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

    #[tokio::test]
    async fn test_shared_map_with_map_asynchronous_execution() {
        let map = SharedMap::new();

        let count = 1000;

        let futures = (0..count).map(|_| {
            let map = map.clone();
            spawn(async move {
                let _ = map.with_map(|map| {
                    let value = map.entry("foo").or_insert(0);
                    *value += 1;
                });
            })
        });

        for future in futures {
            let _ = future.await;
        }

        assert_eq!(map.get(&"foo"), Some(count))
    }
}
