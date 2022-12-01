pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len > 1 {
        _quick_sort(arr, 0, (len - 1) as isize);
    }
}

fn _quick_sort<T: Ord>(arr: &mut [T], lo: isize, hi: isize) {
    if lo < hi {
        let p = partition(arr, lo, hi);
        _quick_sort(arr, lo, p - 1);
        _quick_sort(arr, p + 1, hi);
    }
}

fn partition<T: PartialOrd>(arr: &mut [T], lo: isize, hi: isize) -> isize {
    let pivot = hi as usize;
    let mut i = lo - 1;
    let mut j = hi;

    loop {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && arr[j as usize] > arr[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap(i as usize, pivot as usize);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        quick_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        quick_sort(&mut res);
        assert_eq!(res, vec!["a", "bb", "cc", "d"]);
    }

    #[test]
    fn empty() {
        let mut res: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];
        quick_sort(&mut res);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_sorting_five_elements() {
        let mut res = vec![5, 4, 3, 2, 1];
        quick_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4, 5]);
    }
}
