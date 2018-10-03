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

    if &arr[left] != item {
        return None
    }

    return Some(left);
}

pub fn linear_search<T: PartialEq>(item: &T, arr: &[T]) -> Option<usize> {
    for (i, data) in arr.iter().enumerate() {
        if item == data {
            return Some(i);
        }
    }

    return None;
}
