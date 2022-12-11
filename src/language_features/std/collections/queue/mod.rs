#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    #[test]
    fn test() {
        let mut q: VecDeque<usize> = std::collections::VecDeque::new();

        let range = 0..10;

        for i in range.clone() {
            q.push_back(i)
        }

        for i in range.clone() {
            assert_eq!(Some(i), q.pop_front())
        }
    }
}
