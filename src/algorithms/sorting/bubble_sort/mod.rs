use crate::algorithms::sorting::Sort;

pub struct BubbleSort {}

impl<T: Ord> Sort<T> for BubbleSort {
    fn sort(arr: &mut [T]) {
        if arr.is_empty() {
            return;
        }
        let mut sorted = false;
        let mut n = arr.len();
        while !sorted {
            sorted = true;
            for i in 0..n - 1 {
                if arr[i] > arr[i + 1] {
                    arr.swap(i, i + 1);
                    sorted = false;
                }
            }
            n -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn descending() {
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        BubbleSort::sort(&mut ve1);
        assert!(is_sorted(&ve1));
    }

    #[test]
    fn ascending() {
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        BubbleSort::sort(&mut ve2);
        assert!(is_sorted(&ve2));
    }
    #[test]
    fn empty() {
        let mut ve3: Vec<usize> = vec![];
        BubbleSort::sort(&mut ve3);
        assert!(is_sorted(&ve3));
    }
}
