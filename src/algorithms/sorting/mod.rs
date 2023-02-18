//! Implementations of sorting algorithms
pub use bubble_sort::BubbleSort;
pub use insertion_sort::InsertionSort;
pub use quicksort::QuickSort;
mod bubble_sort;
mod insertion_sort;
mod quicksort;

/// Types that implement this trait can sort slices of data
pub trait Sort<T: Ord> {
    /// Sorts a slice of data
    fn sort(arr: &mut [T]);
}

/// Checks if a slice of data is sorted
///
/// # Examples
///
/// ```
/// use lib_wc::sorting::is_sorted;
///
/// let mut arr = [1, 2, 3, 4, 5];
///
/// assert_eq!(true, is_sorted(&mut arr));
///
/// let mut arr = [1, 2, 3, 4, 5, 4];
///
/// assert_eq!(false, is_sorted(&mut arr));
/// ```
pub fn is_sorted<T: Ord>(arr: &[T]) -> bool {
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
