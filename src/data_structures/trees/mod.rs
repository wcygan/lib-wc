pub use basic_tree::BasicTree;
pub use okay_tree::OkayTree;
mod basic_tree;
mod okay_tree;

trait Tree<T: Ord> {
    fn new() -> Self;
    /// inserts a value into the tree
    fn insert(&mut self, value: T);
    /// searches for a value in the tree
    fn contains(&self, value: T) -> bool;
    /// attempts to remove a value from the tree
    fn remove(&mut self, value: T) -> Option<T>;
}
