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
    use super::*;

    macro_rules! ascending_tests {
        ($($name:ident: $input:expr, $expected:expr;)*) => {
            $(
                #[test]
                fn $name() {
                    let mut arr = $input.clone();
                    heap_sort(&mut arr, true);
                    assert_eq!(arr, $expected);
                }
            )*
        };
    }

    macro_rules! descending_tests {
        ($($name:ident: $input:expr, $expected:expr;)*) => {
            $(
                #[test]
                fn $name() {
                    let mut arr = $input.clone();
                    heap_sort(&mut arr, false);
                    assert_eq!(arr, $expected);
                }
            )*
        };
    }

    ascending_tests! {
        test_empty_vector_ascending: Vec::<i32>::new(), vec![];
        test_single_element_vector_ascending: vec![5], vec![5];
        test_sorted_vector_ascending: vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5];
        test_unsorted_vector_ascending: vec![5, 3, 9, 2, 7], vec![2, 3, 5, 7, 9];
        test_odd_elements_vector_ascending: vec![8, 3, 1, 5, 7], vec![1, 3, 5, 7, 8];
        test_repeated_elements_vector_ascending: vec![5, 5, 5, 5, 5], vec![5, 5, 5, 5, 5];
    }

    descending_tests! {
        test_empty_vector_descending: Vec::<i32>::new(), vec![];
        test_single_element_vector_descending: vec![5], vec![5];
        test_sorted_vector_descending: vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1];
        test_unsorted_vector_descending: vec![5, 3, 9, 2, 7], vec![9, 7, 5, 3, 2];
        test_odd_elements_vector_descending: vec![8, 3, 1, 5, 7], vec![8, 7, 5, 3, 1];
        test_repeated_elements_vector_descending: vec![5, 5, 5, 5, 5], vec![5, 5, 5, 5, 5];
    }
}
