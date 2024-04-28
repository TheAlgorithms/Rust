//! This module provides functions for heap sort algorithm.

use std::cmp::Ordering;

/// Builds a heap from the provided array.
///
/// This function builds either a max heap or a min heap based on the `is_max_heap` parameter.
///
/// # Arguments
///
/// * `arr` - A mutable reference to the array to be sorted.
/// * `is_max_heap` - A boolean indicating whether to build a max heap (`true`) or a min heap (`false`).
fn build_heap<T: Ord>(arr: &mut [T], is_max_heap: bool) {
    let mut i = (arr.len() - 1) / 2;
    while i > 0 {
        heapify(arr, i, is_max_heap);
        i -= 1;
    }
    heapify(arr, 0, is_max_heap);
}

/// Fixes a heap violation starting at the given index.
///
/// This function adjusts the heap rooted at index `i` to fix the heap property violation.
/// It assumes that the subtrees rooted at left and right children of `i` are already heaps.
///
/// # Arguments
///
/// * `arr` - A mutable reference to the array representing the heap.
/// * `i` - The index to start fixing the heap violation.
/// * `is_max_heap` - A boolean indicating whether to maintain a max heap or a min heap.
fn heapify<T: Ord>(arr: &mut [T], i: usize, is_max_heap: bool) {
    let mut comparator: fn(&T, &T) -> Ordering = |a, b| a.cmp(b);
    if !is_max_heap {
        comparator = |a, b| b.cmp(a);
    }

    let mut idx = i;
    let l = 2 * i + 1;
    let r = 2 * i + 2;

    if l < arr.len() && comparator(&arr[l], &arr[idx]) == Ordering::Greater {
        idx = l;
    }

    if r < arr.len() && comparator(&arr[r], &arr[idx]) == Ordering::Greater {
        idx = r;
    }

    if idx != i {
        arr.swap(i, idx);
        heapify(arr, idx, is_max_heap);
    }
}

/// Sorts the given array using heap sort algorithm.
///
/// This function sorts the array either in ascending or descending order based on the `ascending` parameter.
///
/// # Arguments
///
/// * `arr` - A mutable reference to the array to be sorted.
/// * `ascending` - A boolean indicating whether to sort in ascending order (`true`) or descending order (`false`).
pub fn heap_sort<T: Ord>(arr: &mut [T], ascending: bool) {
    if arr.len() <= 1 {
        return;
    }

    // Build heap based on the order
    build_heap(arr, ascending);

    let mut end = arr.len() - 1;
    while end > 0 {
        arr.swap(0, end);
        heapify(&mut arr[..end], 0, ascending);
        end -= 1;
    }
}

#[cfg(test)]
mod tests {
    const HEAP_SORT_TEST_CASES: &[(&[isize], &[isize])] = &[
        (&[], &[]),
        (&[5], &[5]),
        (&[1, 2, 3, 4, 5], &[1, 2, 3, 4, 5]),
        (&[5, 3, 9, 2, 7], &[2, 3, 5, 7, 9]),
        (&[8, 3, 1, 5, 7], &[1, 3, 5, 7, 8]),
        (&[5, 5, 5, 5, 5], &[5, 5, 5, 5, 5]),
    ];

    macro_rules! heap_sort_tests {
        ($function:ident) => {
            mod $function {
                use super::*;

                fn run_test_case(input: &[isize], expected_output: &[isize]) {
                    let mut arr_asc = input.to_vec();
                    super::super::$function(&mut arr_asc, true);
                    assert_eq!(arr_asc, expected_output);

                    let mut arr_desc = input.to_vec();
                    super::super::$function(&mut arr_desc, false);
                    assert_eq!(
                        arr_desc,
                        expected_output.iter().rev().copied().collect::<Vec<_>>()
                    );
                }

                #[test]
                fn test_heap_sort() {
                    for &(input, expected_output) in HEAP_SORT_TEST_CASES.iter() {
                        run_test_case(input, expected_output);
                    }
                }
            }
        };
    }

    heap_sort_tests!(heap_sort);
}
