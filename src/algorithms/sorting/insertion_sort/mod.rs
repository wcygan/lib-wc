pub struct InsertionSort {}
use crate::algorithms::sorting::Sort;

impl<T: Ord> Sort<T> for InsertionSort {
    fn sort(arr: &mut [T]) {
        let len = arr.len();
        for i in 1..len {
            let mut j = i;
            while j > 0 && arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
                j -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::sorting::is_sorted;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn is_sorted_property(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        InsertionSort::sort(&mut arr);
        is_sorted(&mut arr)
    }
}
