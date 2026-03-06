//! # Smallest Range Covering Elements from K Lists
//!
//! Given `k` sorted integer lists, finds the smallest range `[lo, hi]` such
//! that at least one element from every list lies within that range.
//!
//! ## Algorithm
//!
//! A min-heap is seeded with the first element of each list.  On every
//! iteration the heap yields the current global minimum; the global maximum is
//! maintained separately.  If `[min, max]` is tighter than the best range seen
//! so far, it is recorded.  The minimum is then replaced by the next element
//! from the same list.  The loop stops as soon as any list is exhausted,
//! because no further range can cover all lists.
//!
//! ## References
//!
//! - <https://en.wikipedia.org/wiki/Priority_queue>

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Finds the smallest range that includes at least one number from each of the
/// given sorted lists.
///
/// Time complexity: `O(n log k)` where `n` is the total number of elements
/// and `k` is the number of lists.
///
/// Space complexity: `O(k)` for the heap.
///
/// Returns `None` if any list is empty.
pub fn smallest_range(nums: &[&[i64]]) -> Option<[i64; 2]> {
    // A range cannot cover an empty list
    if nums.iter().any(|list| list.is_empty()) {
        return None;
    }

    // Heap entries: (Reverse(value), list_index, element_index).
    // Wrapping the value in Reverse turns BinaryHeap (max-heap) into a min-heap.
    let mut heap: BinaryHeap<(Reverse<i64>, usize, usize)> = BinaryHeap::new();
    let mut current_max = i64::MIN;

    // Seed the heap with the first element from each list
    for (list_idx, list) in nums.iter().enumerate() {
        heap.push((Reverse(list[0]), list_idx, 0));
        current_max = current_max.max(list[0]);
    }

    // Use Option to avoid sentinel arithmetic that could overflow
    let mut best: Option<[i64; 2]> = None;

    let is_tighter = |candidate: [i64; 2], best: Option<[i64; 2]>| match best {
        None => true,
        Some(b) => (candidate[1] - candidate[0]) < (b[1] - b[0]),
    };

    while let Some((Reverse(current_min), list_idx, elem_idx)) = heap.pop() {
        // Check if [current_min, current_max] beats the best range seen so far
        let candidate = [current_min, current_max];
        if is_tighter(candidate, best) {
            best = Some(candidate);
        }

        // If this list is exhausted we can no longer cover all lists
        let next_idx = elem_idx + 1;
        if next_idx == nums[list_idx].len() {
            break;
        }

        // Advance to the next element in the same list
        let next_val = nums[list_idx][next_idx];
        heap.push((Reverse(next_val), list_idx, next_idx));
        current_max = current_max.max(next_val);
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_lists() {
        assert_eq!(
            smallest_range(&[&[4, 10, 15, 24, 26], &[0, 9, 12, 20], &[5, 18, 22, 30]]),
            Some([20, 24])
        );
    }

    #[test]
    fn identical_lists() {
        assert_eq!(
            smallest_range(&[&[1, 2, 3], &[1, 2, 3], &[1, 2, 3]]),
            Some([1, 1])
        );
    }

    #[test]
    fn negative_and_positive() {
        assert_eq!(
            smallest_range(&[&[-3, -2, -1], &[0, 0, 0], &[1, 2, 3]]),
            Some([-1, 1])
        );
    }

    #[test]
    fn non_overlapping() {
        assert_eq!(
            smallest_range(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]),
            Some([3, 7])
        );
    }

    #[test]
    fn all_zeros() {
        assert_eq!(
            smallest_range(&[&[0, 0, 0], &[0, 0, 0], &[0, 0, 0]]),
            Some([0, 0])
        );
    }

    #[test]
    fn empty_lists() {
        assert_eq!(smallest_range(&[&[], &[], &[]]), None);
    }

    #[test]
    fn single_elements() {
        assert_eq!(smallest_range(&[&[5], &[3], &[9]]), Some([3, 9]));
    }

    #[test]
    fn single_list() {
        assert_eq!(smallest_range(&[&[1, 2, 3]]), Some([1, 1]));
    }

    #[test]
    fn one_empty_among_non_empty() {
        assert_eq!(smallest_range(&[&[1, 2], &[], &[3, 4]]), None);
    }
}
