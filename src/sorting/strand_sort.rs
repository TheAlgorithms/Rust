//! # Strand Sort
//!
//! Strand Sort is a comparison-based sorting algorithm that works by repeatedly
//! extracting increasing subsequences ("strands") from the input and merging
//! them into a growing result list.
//!
//! ## Algorithm
//! 1. Remove the first element of the remaining input and start a new *strand*.
//! 2. Scan the rest of the input left-to-right; whenever an element is ≥ the
//!    last element of the strand, pull it out of the input and append it to the
//!    strand.  One full pass yields one sorted strand.
//! 3. Merge the strand into the accumulated result via a standard two-way merge.
//! 4. Repeat until the input is empty.
//!
//! ## Complexity
//!
//! | Case    | Time   | Space |
//! |---------|--------|-------|
//! | Best    | O(n)   | O(n)  |
//! | Average | O(n²)  | O(n)  |
//! | Worst   | O(n²)  | O(n)  |
//!
//! The best case occurs when the input is already sorted (one strand, one merge).
//! The worst case occurs when the input is reverse-sorted (n strands of length 1).
//!
//! ## Reference
//! - [Wikipedia: Strand sort](https://en.wikipedia.org/wiki/Strand_sort)

/// Sorts a `Vec` using the Strand Sort algorithm.
///
/// Strand Sort works by repeatedly pulling increasing "strands" (already-ordered
/// subsequences) out of the input and merging them into a growing result list.
///
/// Because the algorithm relies on removing arbitrary elements mid-collection, it
/// operates on a `Vec<T>` rather than a plain slice.  Linked lists would give
/// O(1) removal; `Vec` removal is O(n) per element but keeps the implementation
/// idiomatic and self-contained.
///
/// # Examples
/// ```
/// use the_algorithms_rust::sorting::strand_sort;
///
/// let mut v = vec![5, 1, 4, 2, 0, 9, 6, 3, 8, 7];
/// strand_sort(&mut v);
/// assert_eq!(v, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
/// ```
pub fn strand_sort<T: Ord>(arr: &mut Vec<T>) {
    let mut result: Vec<T> = Vec::new();

    while !arr.is_empty() {
        // --- Build one sorted strand ---
        // Move the first element of `arr` into the strand unconditionally.
        let mut strand: Vec<T> = vec![arr.remove(0)];

        // Walk the remaining input with an explicit index so we can remove
        // elements in-place without cloning.
        let mut i = 0;
        while i < arr.len() {
            // strand is never empty: it starts with one element and only grows.
            if arr[i] >= *strand.last().unwrap() {
                strand.push(arr.remove(i));
                // `i` now points at the next unvisited element — do NOT advance.
            } else {
                i += 1;
            }
        }

        // --- Merge the strand into the accumulated result ---
        result = merge_sorted(result, strand);
    }

    *arr = result;
}

/// Merges two sorted `Vec`s into a single sorted `Vec`.
///
/// Consumes both inputs and produces a new vector whose length equals the sum
/// of the two input lengths.  This is the standard two-way merge used in
/// merge sort, adapted here for `Vec` ownership.
fn merge_sorted<T: Ord>(left: Vec<T>, right: Vec<T>) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut left = left.into_iter().peekable();
    let mut right = right.into_iter().peekable();

    loop {
        match (left.peek(), right.peek()) {
            (Some(l), Some(r)) => {
                if l <= r {
                    result.push(left.next().unwrap());
                } else {
                    result.push(right.next().unwrap());
                }
            }
            (Some(_), None) => {
                result.extend(left);
                break;
            }
            (None, Some(_)) => {
                result.extend(right);
                break;
            }
            (None, None) => break,
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        let cloned = res.clone();
        strand_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["d", "a", "c", "b"];
        let cloned = res.clone();
        strand_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn empty() {
        let mut res: Vec<i32> = vec![];
        let cloned = res.clone();
        strand_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn one_element() {
        let mut res = vec![42];
        let cloned = res.clone();
        strand_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn already_sorted() {
        let mut res = vec![1, 2, 3, 4, 5];
        let cloned = res.clone();
        strand_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![5, 4, 3, 2, 1];
        let cloned = res.clone();
        strand_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn all_equal() {
        let mut res = vec![7, 7, 7, 7];
        let cloned = res.clone();
        strand_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn duplicates() {
        let mut res = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let cloned = res.clone();
        strand_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    /// Wikipedia's own worked example: {5,1,4,2,0,9,6,3,8,7} → {0..9}
    #[test]
    fn wikipedia_example() {
        let mut res = vec![5, 1, 4, 2, 0, 9, 6, 3, 8, 7];
        strand_sort(&mut res);
        assert_eq!(res, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn negative_numbers() {
        let mut res = vec![-3, -1, -4, -1, -5, -9, -2, -6];
        let cloned = res.clone();
        strand_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }
}
