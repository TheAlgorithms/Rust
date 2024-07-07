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
    let comparator: fn(&T, &T) -> Ordering = if !is_max_heap {
        |a, b| b.cmp(a)
    } else {
        |a, b| a.cmp(b)
    };

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
    use crate::sorting::{have_same_elements, heap_sort, is_descending_sorted, is_sorted};

    macro_rules! test_heap_sort {
        ($($name:ident: $input:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let input_array = $input;
                    let mut arr_asc = input_array.clone();
                    heap_sort(&mut arr_asc, true);
                    assert!(is_sorted(&arr_asc) && have_same_elements(&arr_asc, &input_array));

                    let mut arr_dsc = input_array.clone();
                    heap_sort(&mut arr_dsc, false);
                    assert!(is_descending_sorted(&arr_dsc) && have_same_elements(&arr_dsc, &input_array));
                }
            )*
        }
    }

    test_heap_sort! {
        empty_array: Vec::<i32>::new(),
        single_element_array: vec![5],
        sorted: vec![1, 2, 3, 4, 5],
        sorted_desc: vec![5, 4, 3, 2, 1, 0],
        basic_0: vec![9, 8, 7, 6, 5],
        basic_1: vec![8, 3, 1, 5, 7],
        basic_2: vec![4, 5, 7, 1, 2, 3, 2, 8, 5, 4, 9, 9, 100, 1, 2, 3, 6, 4, 3],
        duplicated_elements: vec![5, 5, 5, 5, 5],
        strings: vec!["aa", "a", "ba", "ab"],
    }
}
