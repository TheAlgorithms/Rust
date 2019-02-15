use std::cmp::{PartialEq, PartialOrd};

pub fn binary_search<T: PartialEq + PartialOrd>(item: &T, arr: &[T]) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;

        if &arr[mid] > item {
            right = mid - 1;
        } else if &arr[mid] < item {
            left = mid + 1;
        } else {
            left = mid;
            break;
        }
    }

    if &arr[left] == item {
        Some(left)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn binary() {
        let index = super::binary_search(&"a", &vec!["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));

        let index = super::binary_search(&4, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = super::binary_search(&3, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = super::binary_search(&2, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = super::binary_search(&1, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(0));

        let index = super::binary_search(&5, &vec![1, 2, 3, 4]);
        assert_eq!(index, None);
    }
}
