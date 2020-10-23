/// Stooge Sort
///
/// Stooge sort is a recursive sorting algorithm. It is notable for its exceptionally bad time complexity of O(n ^ (log 3 / log 1.5)) = O(n2.7095...).

pub fn stooge_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() == 0 {
        return;
    }

    _stooge_sort(arr, 0, arr.len() - 1);
}

fn _stooge_sort<T: Ord>(arr: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    }

    if arr[low] > arr[high] {
        arr.swap(low, high);
    }

    if high - low + 1 > 2 {
        let t = (high - low + 1) / 3;
        _stooge_sort(arr, low, high - t);
        _stooge_sort(arr, low + t, high);
        _stooge_sort(arr, low, high - t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stooge_sort_basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        stooge_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn stooge_sort_basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        stooge_sort(&mut res);
        assert_eq!(res, vec!["a", "bb", "cc", "d"]);
    }

    #[test]
    fn stooge_sort_empty() {
        let mut res = Vec::<u8>::new();
        stooge_sort(&mut res);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn stooge_sort_one_element() {
        let mut res = vec![1];
        stooge_sort(&mut res);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn stooge_sort_pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        stooge_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn stooge_sort_reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        stooge_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }
}
