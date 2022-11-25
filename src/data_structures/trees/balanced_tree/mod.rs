pub struct BalancedTree<K, V>
where
    K: Ord,
    V: Default,
{
    root: Option<Box<Node<K, V>>>,
}

struct Node<K, V>
where
    K: Ord,
    V: Default,
{
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V>
where
    K: Ord,
    V: Default,
{
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
        V: Default,
    {
        if key < self.key {
            match self.left {
                Some(ref mut left) => left.insert(key, value),
                None => {
                    self.left = Some(Box::new(Node::new(key, value)));
                    None
                }
            }
        } else if key > self.key {
            match self.right {
                Some(ref mut right) => right.insert(key, value),
                None => {
                    self.right = Some(Box::new(Node::new(key, value)));
                    None
                }
            }
        } else {
            Some(std::mem::replace(&mut self.value, value))
        }
    }

    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: Ord,
        V: Default,
    {
        if key < &self.key {
            match self.left {
                Some(ref left) => left.get(key),
                None => None,
            }
        } else if key > &self.key {
            match self.right {
                Some(ref right) => right.get(key),
                None => None,
            }
        } else {
            Some(&self.value)
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V>
    where
        K: Ord,
        V: Default,
    {
        if key < &self.key {
            match self.left {
                Some(ref mut left) => left.get_mut(key),
                None => None,
            }
        } else if key > &self.key {
            match self.right {
                Some(ref mut right) => right.get_mut(key),
                None => None,
            }
        } else {
            Some(&mut self.value)
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V>
    where
        K: Ord,
        V: Default,
    {
        if key < &self.key {
            match self.left {
                Some(ref mut left) => left.remove(key),
                None => None,
            }
        } else if key > &self.key {
            match self.right {
                Some(ref mut right) => right.remove(key),
                None => None,
            }
        } else {
            let value = std::mem::replace(&mut self.value, V::default());
            if self.left.is_none() {
                self.right = None;
            } else if self.right.is_none() {
                self.left = None;
            } else {
                let mut right = self.right.take().unwrap();
                let mut right_left = right.left.take();
                while right_left.is_some() {
                    right = right_left.take().unwrap();
                    right_left = right.left.take();
                }
                self.key = right.key;
                self.value = right.value;
                self.right = right.right;
            }
            Some(value)
        }
    }

    pub fn min(&self) -> &K {
        match self.left {
            Some(ref left) => left.min(),
            None => &self.key,
        }
    }

    pub fn max(&self) -> &K {
        match self.right {
            Some(ref right) => right.max(),
            None => &self.key,
        }
    }

    pub fn height(&self) -> usize {
        let left_height = match self.left {
            Some(ref left) => left.height(),
            None => 0,
        };
        let right_height = match self.right {
            Some(ref right) => right.height(),
            None => 0,
        };
        std::cmp::max(left_height, right_height) + 1
    }

    pub fn is_balanced(&self) -> bool {
        let left_height = match self.left {
            Some(ref left) => left.height(),
            None => 0,
        };
        let right_height = match self.right {
            Some(ref right) => right.height(),
            None => 0,
        };

        let diff = left_height as i32 - right_height as i32;

        diff.abs() <= 1
    }
}

impl<K, V> BalancedTree<K, V>
where
    K: Ord,
    V: Default,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
        V: Default,
    {
        match self.root {
            Some(ref mut root) => root.insert(key, value),
            None => {
                self.root = Some(Box::new(Node::new(key, value)));
                None
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: Ord,
        V: Default,
    {
        match self.root {
            Some(ref root) => root.get(key),
            None => None,
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V>
    where
        K: Ord,
        V: Default,
    {
        match self.root {
            Some(ref mut root) => root.get_mut(key),
            None => None,
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V>
    where
        K: Ord,
        V: Default,
    {
        match self.root {
            Some(ref mut root) => root.remove(key),
            None => None,
        }
    }

    pub fn min(&self) -> Option<&K> {
        match self.root {
            Some(ref root) => Some(root.min()),
            None => None,
        }
    }

    pub fn max(&self) -> Option<&K> {
        match self.root {
            Some(ref root) => Some(root.max()),
            None => None,
        }
    }

    pub fn height(&self) -> usize {
        match self.root {
            Some(ref root) => root.height(),
            None => 0,
        }
    }

    pub fn is_balanced(&self) -> bool {
        match self.root {
            Some(ref root) => root.is_balanced(),
            None => true,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_five() {
        let mut tree = BalancedTree::new();
        tree.insert(5, 5);
        assert_eq!(tree.get(&5), Some(&5));
        assert_eq!(tree.get(&4), None);
        assert_eq!(tree.get(&6), None);
        assert_eq!(tree.min(), Some(&5));
        assert_eq!(tree.max(), Some(&5));
        assert_eq!(tree.height(), 1);
        assert_eq!(tree.is_balanced(), true);
    }

    #[test]
    fn test_is_balanced() {
        let mut tree = BalancedTree::new();
        tree.insert(5, 5);
        tree.insert(4, 4);
        tree.insert(6, 6);
        assert_eq!(tree.is_balanced(), true);
        tree.insert(3, 3);
        assert_eq!(tree.is_balanced(), true);
    }

    #[test]
    fn test_height() {
        let mut tree = BalancedTree::new();
        tree.insert(5, 5);
        tree.insert(4, 4);
        tree.insert(6, 6);
        assert_eq!(tree.height(), 2);
        tree.insert(3, 3);
        assert_eq!(tree.height(), 3);
    }

    #[test]
    fn test_get_mut() {
        let mut tree = BalancedTree::new();
        tree.insert(5, 5);
        tree.insert(4, 4);
        tree.insert(6, 6);
        assert_eq!(tree.get_mut(&5), Some(&mut 5));
        assert_eq!(tree.get_mut(&4), Some(&mut 4));
        assert_eq!(tree.get_mut(&6), Some(&mut 6));
        assert_eq!(tree.get_mut(&3), None);
    }

    #[test]
    fn test_min() {
        let mut tree = BalancedTree::new();
        tree.insert(5, 5);
        tree.insert(4, 4);
        tree.insert(6, 6);
        assert_eq!(tree.min(), Some(&4));
    }

    #[test]
    fn test_max() {
        let mut tree = BalancedTree::new();
        tree.insert(5, 5);
        tree.insert(4, 4);
        tree.insert(6, 6);
        assert_eq!(tree.max(), Some(&6));
    }

    #[test]
    fn test_remove() {
        let mut tree = BalancedTree::new();
        tree.insert(5, 5);
        tree.insert(4, 4);
        tree.insert(6, 6);
        assert_eq!(tree.remove(&5), Some(5));
        assert_eq!(tree.remove(&4), Some(4));
        assert_eq!(tree.remove(&6), Some(6));
        assert_eq!(tree.remove(&3), None);
    }
}
