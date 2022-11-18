///
/// This is just a wrapper around Vec
///
struct BasicList<T> {
    vec: Vec<T>,
}

impl<T> BasicList<T> {
    fn new() -> Self {
        BasicList { vec: vec![] }
    }

    fn add(&mut self, data: T) {
        self.vec.push(data)
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.vec.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one() {
        let mut lst: BasicList<u32> = BasicList::new();

        let input = 25;
        lst.add(input);

        let result = lst.get(0);

        match result {
            None => {
                panic!()
            }
            Some(output) => {
                assert_eq!(*output, input)
            }
        }
    }

    #[test]
    fn add_multiple() {
        let mut lst: BasicList<u32> = BasicList::new();

        let vec: Vec<u32> = (0..100).collect();

        for num in &vec {
            lst.add(*num)
        }

        for i in 0..vec.len() {
            let expected = vec.get(i);
            let actual = lst.get(i);

            match (expected, actual) {
                (Some(a), Some(b)) => {
                    assert_eq!(a, b)
                }
                (_, _) => {
                    panic!()
                }
            }
        }
    }
}
