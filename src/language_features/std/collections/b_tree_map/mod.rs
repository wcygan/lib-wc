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
}
