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
        Tree::new()
    }
}

impl<T> Tree<T>
where
    T: Ord,
{
    /// create a new Tree
    pub fn new() -> Tree<T> {
        Self {
            value: None,
            left: None,
            right: None
        }
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

    #[test]
    fn test_minimum() {
        let mut t = Tree::<u32>::default();

        t.insert(1);
        t.insert(2);
        t.insert(3);

        match t.minimum() {
            None => {
                panic!("should not hit this branch")
            }
            Some(val) => {
                assert_eq!(1, *val)
            }
        }
    }

    #[test]
    fn test_maximum() {
        let mut t = Tree::<u32>::default();

        t.insert(1);
        t.insert(2);
        t.insert(3);

        match t.maximum() {
            None => {
                panic!("should not hit this branch")
            }
            Some(val) => {
                assert_eq!(3, *val)
            }
        }
    }

    #[test]
    fn search_for_value_in_tree() {
        let mut t = Tree::<u32>::default();

        t.insert(1);

        assert_eq!(t.search(&1), true)
    }

    #[test]
    fn search_for_value_not_in_tree() {
        let mut t = Tree::<u32>::default();

        assert_eq!(t.search(&1), false)
    }

    #[test]
    fn insert_does_not_panic() {
        let mut t = Tree::<u32>::default();

        for i in 0..10 {
            t.insert(i)
        }
    }

    #[test]
    fn search_does_not_panic() {
        let mut t = Tree::<u32>::default();

        for i in 0..10 {
            t.insert(i);
            t.search(&i);
        }
    }
}
