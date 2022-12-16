#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    #[test]
    fn insert() {
        let mut t: BTreeMap<usize, usize> = BTreeMap::new();
        assert_eq!(false, t.contains_key(&1));
        t.insert(1, 2);
        assert_eq!(true, t.contains_key(&1));
    }

    #[test]
    fn btree_five() {
        let mut t: BTreeMap<usize, usize> = BTreeMap::new();
        t.insert(1, 2);
        t.insert(2, 3);
        t.insert(3, 4);
        t.insert(4, 5);
        t.insert(5, 6);
        assert_eq!(Some(&2), t.get(&1));
        assert_eq!(Some(&3), t.get(&2));
        assert_eq!(Some(&4), t.get(&3));
        assert_eq!(Some(&5), t.get(&4));
        assert_eq!(Some(&6), t.get(&5));
    }

    #[test]
    fn btree_min() {
        let mut t: BTreeMap<usize, usize> = BTreeMap::new();
        t.insert(1, 2);
        t.insert(2, 3);
        t.insert(3, 4);
        t.insert(4, 5);
        t.insert(5, 6);
        assert_eq!(Some((&1, &2)), t.first_key_value());
    }

    #[test]
    fn btree_max() {
        let mut t: BTreeMap<usize, usize> = BTreeMap::new();
        t.insert(1, 2);
        t.insert(2, 3);
        t.insert(3, 4);
        t.insert(4, 5);
        t.insert(5, 6);
        assert_eq!(Some((&5, &6)), t.last_key_value());
    }
}
