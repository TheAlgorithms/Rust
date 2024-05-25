//! This module implements the Pattern-Defeating Quicksort (PDQSort) algorithm.
//!
//! PDQSort is an optimized version of Quicksort that includes enhancements to deal with
//! various patterns in the input data, such as already sorted or nearly sorted sequences.
//! It dynamically switches between different sorting strategies based on the characteristics
//! of the data and recursion depth to achieve better performance.
//!
//! PDQSort employs the following techniques:
//! - Insertion Sort for small arrays to reduce overhead.
//! - Partitioning around a pivot to divide the array into subarrays.
//! - Heap Sort as a fallback when the recursion depth limit is reached, preventing worst-case
//!   scenarios that could degrade performance to O(n^2).
//! - Tail call optimization to avoid excessive recursive calls and stack overflow.

/// Sorts a portion of the array using insertion sort.
///
/// # Parameters
///
/// - `arr`: The list to sort.
/// - `left`: The starting index of the portion to sort.
/// - `right`: The ending index of the portion to sort.
fn insertion_sort<T: Ord>(arr: &mut [T], left: usize, right: usize) {
    for idx in left + 1..=right {
        let mut curr = idx;
        while curr > left && arr[curr] < arr[curr - 1] {
            arr.swap(curr, curr - 1);
            curr -= 1;
        }
    }
}

/// Partitions the array around a pivot.
///
/// # Parameters
///
/// - `arr`: The list to partition.
/// - `low`: The starting index of the portion to partition.
/// - `high`: The ending index of the portion to partition.
///
/// # Returns
///
/// The index of the pivot.
fn partition<T: Ord>(arr: &mut [T], low: usize, high: usize) -> usize {
    let pivot = high;
    let mut curr = low;

    for idx in low..high {
        if arr[idx] < arr[pivot] {
            arr.swap(curr, idx);
            curr += 1;
        }
    }
    arr.swap(curr, pivot);
    curr
}

/// Converts a portion of the array into a heap.
///
/// # Parameters
///
/// - `arr`: The list to heapify.
/// - `heap_size`: The size of the heap.
/// - `node_idx`: The index of the current node.
/// - `low`: The starting index of the portion to heapify.
fn heapify<T: Ord>(arr: &mut [T], heap_size: usize, node_idx: usize, low: usize) {
    let mut largest = node_idx;
    let left = 2 * node_idx + 1;
    let right = 2 * node_idx + 2;

    if left < heap_size && arr[low + left] > arr[low + largest] {
        largest = left;
    }

    if right < heap_size && arr[low + right] > arr[low + largest] {
        largest = right;
    }

    if largest != node_idx {
        arr.swap(low + node_idx, low + largest);
        heapify(arr, heap_size, largest, low);
    }
}

/// Sorts a portion of the array using heap sort.
///
/// # Parameters
///
/// - `arr`: The list to sort.
/// - `low`: The starting index of the portion to sort.
/// - `high`: The ending index of the portion to sort.
fn heap_sort<T: Ord>(arr: &mut [T], low: usize, high: usize) {
    let heap_size = high - low + 1;

    for node_idx in (0..heap_size / 2).rev() {
        heapify(arr, heap_size, node_idx, low);
    }

    for size in (1..heap_size).rev() {
        arr.swap(low, low + size);
        heapify(arr, size, 0, low);
    }
}

/// Recursively sorts the array using the PDQSort algorithm.
///
/// # Parameters
///
/// - `arr`: The list to sort.
/// - `low`: The starting index of the portion to sort.
/// - `high`: The ending index of the portion to sort.
/// - `depth_limit`: The maximum recursion depth.
fn pdqsort_recursive<T: Ord>(arr: &mut [T], mut low: usize, high: usize, depth_limit: usize) {
    while low < high {
        // Use insertion sort for small partitions
        if high - low <= 16 {
            insertion_sort(arr, low, high);
            return;
        }

        // Switch to heap sort if depth limit is reached
        if depth_limit == 0 {
            heap_sort(arr, low, high);
            return;
        }

        let pivot_index = partition(arr, low, high);
        if pivot_index > 0 {
            pdqsort_recursive(arr, low, pivot_index - 1, depth_limit - 1);
        }

        // Tail recursion elimination
        low = pivot_index + 1;
    }
}

/// Sorts the entire array using the PDQSort algorithm.
///
/// # Parameters
///
/// - `arr`: The list to sort.
pub fn pdqsort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let max_depth = (arr.len().trailing_zeros() as usize) * 2;
    pdqsort_recursive(arr, 0, arr.len() - 1, max_depth);
}

#[cfg(test)]
mod tests {
    use crate::sorting::{have_same_elements, is_sorted, pdqsort};
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    macro_rules! test_pdqsort {
        ($($name:ident: $input:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let mut arr = $input;
                    let input_arr = arr.clone();
                    pdqsort(&mut arr);
                    assert!(is_sorted(&arr) && have_same_elements(&arr, &input_arr));
                }
            )*
        }
    }

    test_pdqsort! {
        test_empty_arr: Vec::<i32>::new(),
        test_single_element_arr: vec![1],
        test_sorted_arr: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        test_reverse_sorted_arr: vec![10, 9, 8, 7, 6, 6, 5, 4, 3, 2, 1],
        test_random_arr: vec![5, 2, 8, 3, 1, 9, 4, 6, 7, 10],
        test_duplicates: vec![3, 2, 1, 3, 1, 2, 7, 5, 6, 5, 10, 10],
        test_empty_str_arr: Vec::<&str>::new(),
        test_single_str_arr: vec!["apple"],
        test_sorted_str_arr: vec!["apple", "banana", "grape", "kiwi", "orange"],
        test_reverse_sorted_str_arr: vec!["orange", "kiwi", "grape", "banana", "apple"],
        test_random_str_array: vec!["banana", "apple", "orange", "grape", "kiwi"],
        test_arr_contain_empty_str: vec!["", "", "apple", "", "orange", "", "kiwi", "grape", "banana", ""],
    }

    #[test]
    fn test_large_random_array() {
        let mut rng = thread_rng();
        let mut arr: Vec<i32> = (0..100_000).collect();
        arr.shuffle(&mut rng);
        let input_arr = arr.clone();
        pdqsort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &input_arr));
    }
}
