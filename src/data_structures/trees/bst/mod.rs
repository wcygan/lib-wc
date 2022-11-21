/// this is a very simple binary search Tree :)
/// copied from the-algorithms/rust
pub struct Tree<T>
where
    T: Ord,
{
    value: Option<T>,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
}

impl<T> Default for Tree<T>
where
    T: Ord,
{
    fn default() -> Self {
        todo!()
    }
}

impl<T> Tree<T>
where
    T: Ord,
{
    /// create a new Tree
    pub fn new() -> Tree<T> {
        todo!()
    }

    /// search for a value in the Tree.
    /// returns true iff the value exists in the Tree.
    pub fn search(&self, value: &T) -> bool {
        todo!()
    }

    /// insert a value into the Tree
    pub fn insert(&mut self, value: T) {
        todo!()
    }

    /// finds the minimum value of the Tree if it exists, else None
    pub fn minimum(&self) -> Option<&T> {
        todo!()
    }

    /// finds the maximum value of the Tree if it exists, else None
    pub fn maximum(&self) -> Option<&T> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let t = Tree::<u32>::default();
    }
}
