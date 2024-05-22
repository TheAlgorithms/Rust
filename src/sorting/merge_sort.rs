//! This module provides implementations of merge sort using both top-down and bottom-up approaches.

/// Merges two sorted subarrays into a single sorted array.
///
/// The `merge` function takes a mutable slice `arr` and an index `mid` which splits the slice into
/// two subarrays: `arr[..mid]` and `arr[mid..]`. These subarrays are then merged into a single
/// sorted array.
///
/// # Parameters
///
/// - `arr`: The mutable slice to be sorted.
/// - `mid`: The index at which to split the array into two subarrays.
fn merge<T: Ord + Copy>(arr: &mut [T], mid: usize) {
    let left_half = arr[..mid].to_vec();
    let right_half = arr[mid..].to_vec();

    let mut left = 0;
    let mut right = 0;

    for val in arr {
        if right == right_half.len()
            || (left < left_half.len() && left_half[left] < right_half[right])
        {
            *val = left_half[left];
            left += 1;
        } else {
            *val = right_half[right];
            right += 1;
        }
    }
}

/// Sorts an array using the top-down merge sort algorithm.
///
/// The `top_down_merge_sort` function recursively divides the array into halves, sorts each half,
/// and then merges the sorted halves. This function is a classic implementation of the merge sort
/// algorithm that uses a divide-and-conquer approach.
///
/// # Parameters
///
/// - `arr`: The mutable slice to be sorted.
pub fn top_down_merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        top_down_merge_sort(&mut arr[..mid]);
        top_down_merge_sort(&mut arr[mid..]);
        merge(arr, mid);
    }
}

/// Sorts an array using the bottom-up merge sort algorithm.
///
/// The `bottom_up_merge_sort` function iteratively merges subarrays of increasing size until the
/// entire array is sorted. This function is a non-recursive implementation of the merge sort algorithm
/// that starts with small subarrays and progressively merges larger ones.
///
/// # Parameters
///
/// - `arr`: The mutable slice to be sorted.
pub fn bottom_up_merge_sort<T: Copy + Ord>(arr: &mut [T]) {
    if arr.len() > 1 {
        let mut sub_array_size = 1;
        while sub_array_size < arr.len() {
            for start_index in (0..arr.len()).step_by(2 * sub_array_size) {
                let mid = start_index + sub_array_size;
                if mid < arr.len() {
                    let end = usize::min(start_index + 2 * sub_array_size, arr.len());
                    merge(&mut arr[start_index..end], mid - start_index);
                }
            }
            sub_array_size *= 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    const MERGE_SORT_TEST_CASES_INT: &[&[usize]] = &[
        &[],
        &[10, 8, 4, 3, 1, 9, 2, 7, 5, 6],
        &[1],
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        &[10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
    ];

    const MERGE_SORT_TEST_CASES_STR: &[&[&str]] = &[
        &[""],
        &["banana", "apple", "cherry", "date"],
        &["apple"],
        &["apple", "banana", "cherry", "date"],
        &["date", "cherry", "banana", "apple"],
    ];

    macro_rules! merge_sort_tests {
        ($function:ident) => {
            mod $function {
                use super::*;

                fn run_test_case<T: Ord + Copy + std::hash::Hash>(input: &[T]) {
                    let mut arr = input.to_vec();
                    super::super::$function(&mut arr);
                    assert!(is_sorted(&arr) && have_same_elements(&arr, &input.to_vec()));
                }

                #[test]
                fn test_merge_sort() {
                    for (int_input, str_input) in MERGE_SORT_TEST_CASES_INT
                        .iter()
                        .zip(MERGE_SORT_TEST_CASES_STR.iter())
                    {
                        run_test_case(*int_input);
                        run_test_case(*str_input);
                    }
                }
            }
        };
    }

    merge_sort_tests!(top_down_merge_sort);
    merge_sort_tests!(bottom_up_merge_sort);
}
