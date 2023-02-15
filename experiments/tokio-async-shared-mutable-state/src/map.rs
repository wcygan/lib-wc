use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SharedMap<K, V> {
    inner: Arc<Mutex<SharedMapInner<K, V>>>,
}

struct SharedMapInner<K, V> {
    map: HashMap<K, V>,
}

impl<K, V> SharedMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(SharedMapInner {
                map: HashMap::new(),
            })),
        }
    }

    pub fn insert(&self, key: K, value: V) {
        let mut inner = self.inner.lock().unwrap();
        inner.map.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        let inner = self.inner.lock().unwrap();
        inner.map.get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
