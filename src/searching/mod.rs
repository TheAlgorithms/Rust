use std::cmp;

pub fn binary_search<T>(item: T, arr: &[T]) -> i32
    where T: cmp::PartialEq + cmp::PartialOrd + Sized
{
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;

        if arr[mid] > item {
            right = mid - 1;
        } else if arr[mid] < item {
            left = mid + 1;
        } else {
            left = mid;
            break;
        }
    }

    if arr[left] != item {
        return -1;
    }

    left as i32
}
