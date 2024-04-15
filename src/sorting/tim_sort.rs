//! Implements Tim sort algorithm.
//!
//! Tim sort is a hybrid sorting algorithm derived from merge sort and insertion sort.
//! It is designed to perform well on many kinds of real-world data.

use std::cmp;

static MIN_MERGE: usize = 32;

/// Determines the minimum run length for Tim sort.
///
/// The minimum run length is calculated based on the minimum merge size.
///
/// # Arguments
///
/// * `n` - The length of the array.
///
/// # Returns
///
/// The minimum run length.
fn min_run_length(mut n: usize) -> usize {
    let mut r = 0;
    while n >= MIN_MERGE {
        r |= n & 1;
        n >>= 1;
    }
    n + r
}

/// Sorts a slice using insertion sort algorithm.
///
/// This function sorts the provided slice in-place using the insertion sort algorithm.
///
/// # Arguments
///
/// * `arr` - The slice to be sorted.
fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let temp = arr[i];
        let mut j = i;

        while j > 0 && arr[j - 1] > temp {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = temp;
    }
}

/// Merges two sorted subarrays into a single sorted subarray.
///
/// This function merges two sorted subarrays of the provided slice into a single sorted subarray.
///
/// # Arguments
///
/// * `arr` - The slice containing the subarrays to be merged.
/// * `l` - The starting index of the first subarray.
/// * `m` - The ending index of the first subarray.
/// * `r` - The ending index of the second subarray.
fn merge(arr: &mut [i32], l: usize, m: usize, r: usize) {
    let left = arr[l..=m].to_vec();
    let right = arr[m + 1..=r].to_vec();
    let mut i = 0;
    let mut j = 0;
    let mut k = l;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        k += 1;
        i += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        k += 1;
        j += 1;
    }
}

/// Sorts a slice using Tim sort algorithm.
///
/// This function sorts the provided slice in-place using the Tim sort algorithm.
///
/// # Arguments
///
/// * `arr` - The slice to be sorted.
pub fn tim_sort(arr: &mut [i32]) {
    let n = arr.len();
    let min_run = min_run_length(MIN_MERGE);

    let mut i = 0;
    while i < n {
        insertion_sort(&mut arr[i..cmp::min(i + MIN_MERGE, n)]);
        i += min_run;
    }

    let mut size = min_run;
    while size < n {
        let mut left = 0;
        while left < n {
            let mid = left + size - 1;
            let right = cmp::min(left + 2 * size - 1, n - 1);
            if mid < right {
                merge(arr, left, mid, right);
            }

            left += 2 * size;
        }
        size *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::{have_same_elements, is_sorted};

    #[test]
    fn min_run_length_returns_correct_value() {
        assert_eq!(min_run_length(0), 0);
        assert_eq!(min_run_length(10), 10);
        assert_eq!(min_run_length(33), 17);
        assert_eq!(min_run_length(64), 16);
    }

    #[test]
    fn insertion_sort_sorts_array_correctly() {
        let mut array = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        insertion_sort(&mut array);
        assert_eq!(array, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);

        let mut array = vec![1, 2, 3, 4, 5];
        insertion_sort(&mut array);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn merge_merges_sorted_arrays_correctly() {
        let mut array = vec![1, 3, 5, 2, 4, 6];
        merge(&mut array, 0, 2, 5);
        assert_eq!(array, vec![1, 2, 3, 4, 5, 6]);

        let mut array = vec![1, 2, 3];
        merge(&mut array, 0, 0, 2);
        assert_eq!(array, vec![1, 2, 3]);
    }

    #[test]
    fn tim_sort_sorts_basic_array_correctly() {
        let mut array = vec![-2, 7, 15, -14, 0, 15, 0, 7, -7, -4, -13, 5, 8, -14, 12];
        let cloned = array.clone();
        tim_sort(&mut array);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }

    #[test]
    fn tim_sort_handles_empty_array() {
        let mut array = Vec::<i32>::new();
        let cloned = array.clone();
        tim_sort(&mut array);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }

    #[test]
    fn tim_sort_handles_single_element_array() {
        let mut array = vec![3];
        let cloned = array.clone();
        tim_sort(&mut array);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }

    #[test]
    fn tim_sort_handles_pre_sorted_array() {
        let mut array = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let cloned = array.clone();
        tim_sort(&mut array);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }
}
