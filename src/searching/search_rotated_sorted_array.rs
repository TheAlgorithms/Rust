//! Search for a target in a rotated sorted array.
//!
//! This implementation returns the index of `target` if present, or `None`.
//! It assumes the input slice contains distinct elements and was originally
//! sorted in ascending order before rotation.

/// Searches for `target` in a rotated sorted slice and returns its index.
///
/// Time complexity: O(log n)
pub fn search_rotated_sorted_array<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut left: isize = 0;
    let mut right: isize = (arr.len() - 1) as isize;

    while left <= right {
        let mid = left + (right - left) / 2;
        let mid_usize = mid as usize;

        if &arr[mid_usize] == target {
            return Some(mid_usize);
        }

        // Determine which half is normally ordered
        if arr[left as usize] <= arr[mid_usize] {
            // Left half is sorted
            if &arr[left as usize] <= target && target < &arr[mid_usize] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            // Right half is sorted
            if &arr[mid_usize] < target && target <= &arr[right as usize] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_in_rotated_sorted_array_found_examples() {
        let arr = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(search_rotated_sorted_array(&arr, &0), Some(4));
        assert_eq!(search_rotated_sorted_array(&arr, &4), Some(0));
        assert_eq!(search_rotated_sorted_array(&arr, &2), Some(6));
    }

    #[test]
    fn search_in_rotated_sorted_array_not_found() {
        let arr = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(search_rotated_sorted_array(&arr, &3), None);
    }

    #[test]
    fn empty_and_single() {
        let empty: Vec<i32> = vec![];
        assert_eq!(search_rotated_sorted_array(&empty, &1), None);

        let single = vec![1];
        assert_eq!(search_rotated_sorted_array(&single, &1), Some(0));
        assert_eq!(search_rotated_sorted_array(&single, &2), None);
    }

    #[test]
    fn non_rotated_array() {
        // already sorted ascending
        let arr = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(search_rotated_sorted_array(&arr, &0), Some(0));
        assert_eq!(search_rotated_sorted_array(&arr, &5), Some(5));
        assert_eq!(search_rotated_sorted_array(&arr, &3), Some(3));
        assert_eq!(search_rotated_sorted_array(&arr, &6), None);
    }

    #[test]
    fn small_rotations_and_edges() {
        // rotation by 1
        let arr1 = vec![5, 0, 1, 2, 3, 4];
        assert_eq!(search_rotated_sorted_array(&arr1, &5), Some(0));
        assert_eq!(search_rotated_sorted_array(&arr1, &4), Some(5));

        // rotation by len-1 (same as rotation by -1)
        let arr2 = vec![1, 2, 3, 4, 5, 0];
        assert_eq!(search_rotated_sorted_array(&arr2, &0), Some(5));
        assert_eq!(search_rotated_sorted_array(&arr2, &1), Some(0));
    }

    #[test]
    fn two_elements_varieties() {
        let a = vec![1, 2];
        assert_eq!(search_rotated_sorted_array(&a, &1), Some(0));
        assert_eq!(search_rotated_sorted_array(&a, &2), Some(1));

        let b = vec![2, 1];
        assert_eq!(search_rotated_sorted_array(&b, &1), Some(1));
        assert_eq!(search_rotated_sorted_array(&b, &2), Some(0));
    }
}
