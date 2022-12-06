trait Tree<K: Ord, V> {
    /// Creates a new tree
    fn new() -> Self;
    /// Inserts a value into the tree
    fn insert(&mut self, key: K, value: V);
    /// Searches for a value in the tree
    fn contains(&self, value: K) -> bool;
    /// Attempts to remove a value from the tree
    fn remove(&mut self, value: K) -> Option<V>;
}
