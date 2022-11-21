use std::cmp::Ordering;

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
            right: None,
        }
    }

    /// insert a value into the Tree
    pub fn insert(&mut self, value: T) {
        match &self.value {
            None => self.value = Some(value),
            Some(current) => {
                let target = match value.cmp(current) {
                    Ordering::Less => &mut self.left,
                    Ordering::Equal | Ordering::Greater => &mut self.right,
                };

                match target {
                    None => {
                        let mut node = Tree::default();
                        node.insert(value);
                        *target = Some(Box::new(node));
                    }
                    Some(node) => node.insert(value),
                }
            }
        }
    }

    /// search for a value in the Tree.
    /// returns true iff the value exists in the Tree.
    pub fn search(&self, value: &T) -> bool {
        match &self.value {
            None => false,
            Some(key) => match value.cmp(key) {
                Ordering::Equal => true,
                Ordering::Less => match &self.left {
                    None => false,
                    Some(node) => node.search(value),
                },
                Ordering::Greater => match &self.right {
                    None => false,
                    Some(node) => node.search(value),
                },
            },
        }
    }

    /// finds the minimum value of the Tree if it exists, else None
    pub fn minimum(&self) -> Option<&T> {
        match &self.left {
            None => self.value.as_ref(),
            Some(node) => node.minimum(),
        }
    }

    /// finds the maximum value of the Tree if it exists, else None
    pub fn maximum(&self) -> Option<&T> {
        match &self.right {
            None => self.value.as_ref(),
            Some(node) => node.maximum(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let _t = Tree::<u32>::default();
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
        let t = Tree::<u32>::default();

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
