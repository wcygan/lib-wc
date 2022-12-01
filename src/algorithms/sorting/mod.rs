pub use quicksort::quicksort;
mod quicksort;

pub fn is_sorted<T: Ord>(arr: &mut [T]) -> bool {
    for i in 1..arr.len() {
        if arr[i - 1] > arr[i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        assert_eq!(true, is_sorted(&mut arr));
    }

    #[test]
    fn test_is_not_sorted() {
        let mut arr = [1, 2, 3, 4, 5, 4];
        assert_eq!(false, is_sorted(&mut arr));
    }
}