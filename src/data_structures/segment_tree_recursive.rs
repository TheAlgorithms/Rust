use std::fmt::Debug;
use std::ops::Range;

pub struct SegmentTree<T: Debug + Default + Ord + Copy> {
    len: usize,           // length of the represented
    tree: Vec<T>, // represents a binary tree of intervals as an array (as a BinaryHeap does, for instance)
    merge: fn(T, T) -> T, // how we merge two values together
}

impl<T: Debug + Default + Ord + Copy> SegmentTree<T> {
    pub fn from_vec(arr: &[T], merge: fn(T, T) -> T) -> Self {
        let len = arr.len();
        let mut sgtr = SegmentTree {
            len,
            tree: vec![T::default(); 4 * len],
            merge,
        };
        if len != 0 {
            sgtr.build_recursive(arr, 1, 0..len, merge);
        }
        sgtr
    }

    fn build_recursive(
        &mut self,
        arr: &[T],
        idx: usize,
        range: Range<usize>,
        merge: fn(T, T) -> T,
    ) {
        if range.end - range.start == 1 {
            self.tree[idx] = arr[range.start];
        } else {
            let mid = range.start + (range.end - range.start) / 2;
            self.build_recursive(arr, 2 * idx, range.start..mid, merge);
            self.build_recursive(arr, 2 * idx + 1, mid..range.end, merge);
            self.tree[idx] = merge(self.tree[2 * idx], self.tree[2 * idx + 1]);
        }
    }

    /// Query the range (exclusive)
    /// returns None if the range is out of the array's boundaries (eg: if start is after the end of the array, or start > end, etc.)
    /// return the aggregate of values over this range otherwise
    pub fn query(&self, range: Range<usize>) -> Option<T> {
        self.query_recursive(1, 0..self.len, &range)
    }

    fn query_recursive(
        &self,
        idx: usize,
        element_range: Range<usize>,
        query_range: &Range<usize>,
    ) -> Option<T> {
        if element_range.start >= query_range.end || element_range.end <= query_range.start {
            return None;
        }
        if element_range.start >= query_range.start && element_range.end <= query_range.end {
            return Some(self.tree[idx]);
        }
        let mid = element_range.start + (element_range.end - element_range.start) / 2;
        let left = self.query_recursive(idx * 2, element_range.start..mid, query_range);
        let right = self.query_recursive(idx * 2 + 1, mid..element_range.end, query_range);
        match (left, right) {
            (None, None) => None,
            (None, Some(r)) => Some(r),
            (Some(l), None) => Some(l),
            (Some(l), Some(r)) => Some((self.merge)(l, r)),
        }
    }

    /// Updates the value at index `idx` in the original array with a new value `val`
    pub fn update(&mut self, idx: usize, val: T) {
        self.update_recursive(1, 0..self.len, idx, val);
    }

    fn update_recursive(
        &mut self,
        idx: usize,
        element_range: Range<usize>,
        target_idx: usize,
        val: T,
    ) {
        println!("{element_range:?}");
        if element_range.start > target_idx || element_range.end <= target_idx {
            return;
        }
        if element_range.end - element_range.start <= 1 && element_range.start == target_idx {
            println!("{element_range:?}");
            self.tree[idx] = val;
            return;
        }
        let mid = element_range.start + (element_range.end - element_range.start) / 2;
        self.update_recursive(idx * 2, element_range.start..mid, target_idx, val);
        self.update_recursive(idx * 2 + 1, mid..element_range.end, target_idx, val);
        self.tree[idx] = (self.merge)(self.tree[idx * 2], self.tree[idx * 2 + 1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;
    use std::cmp::{max, min};

    #[test]
    fn test_min_segments() {
        let vec = vec![-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let min_seg_tree = SegmentTree::from_vec(&vec, min);
        assert_eq!(Some(-5), min_seg_tree.query(4..7));
        assert_eq!(Some(-30), min_seg_tree.query(0..vec.len()));
        assert_eq!(Some(-30), min_seg_tree.query(0..2));
        assert_eq!(Some(-4), min_seg_tree.query(1..3));
        assert_eq!(Some(-5), min_seg_tree.query(1..7));
    }

    #[test]
    fn test_max_segments() {
        let val_at_6 = 6;
        let vec = vec![1, 2, -4, 7, 3, -5, val_at_6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut max_seg_tree = SegmentTree::from_vec(&vec, max);
        assert_eq!(Some(15), max_seg_tree.query(0..vec.len()));
        let max_4_to_6 = 6;
        assert_eq!(Some(max_4_to_6), max_seg_tree.query(4..7));
        let delta = 2;
        max_seg_tree.update(6, val_at_6 + delta);
        assert_eq!(Some(val_at_6 + delta), max_seg_tree.query(4..7));
    }

    #[test]
    fn test_sum_segments() {
        let val_at_6 = 6;
        let vec = vec![1, 2, -4, 7, 3, -5, val_at_6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut sum_seg_tree = SegmentTree::from_vec(&vec, |a, b| a + b);
        for (i, val) in vec.iter().enumerate() {
            assert_eq!(Some(*val), sum_seg_tree.query(i..(i + 1)));
        }
        let sum_4_to_6 = sum_seg_tree.query(4..7);
        assert_eq!(Some(4), sum_4_to_6);
        let delta = 3;
        sum_seg_tree.update(6, val_at_6 + delta);
        assert_eq!(
            sum_4_to_6.unwrap() + delta,
            sum_seg_tree.query(4..7).unwrap()
        );
    }

    // Some properties over segment trees:
    //  When asking for the range of the overall array, return the same as iter().min() or iter().max(), etc.
    //  When asking for an interval containing a single value, return this value, no matter the merge function

    #[quickcheck]
    fn check_overall_interval_min(array: Vec<i32>) -> TestResult {
        let seg_tree = SegmentTree::from_vec(&array, min);
        TestResult::from_bool(array.iter().min().copied() == seg_tree.query(0..array.len()))
    }

    #[quickcheck]
    fn check_overall_interval_max(array: Vec<i32>) -> TestResult {
        let seg_tree = SegmentTree::from_vec(&array, max);
        TestResult::from_bool(array.iter().max().copied() == seg_tree.query(0..array.len()))
    }

    #[quickcheck]
    fn check_overall_interval_sum(array: Vec<i32>) -> TestResult {
        let seg_tree = SegmentTree::from_vec(&array, max);
        TestResult::from_bool(array.iter().max().copied() == seg_tree.query(0..array.len()))
    }

    #[quickcheck]
    fn check_single_interval_min(array: Vec<i32>) -> TestResult {
        let seg_tree = SegmentTree::from_vec(&array, min);
        for (i, value) in array.into_iter().enumerate() {
            let res = seg_tree.query(i..(i + 1));
            if res != Some(value) {
                return TestResult::error(format!("Expected {:?}, got {:?}", Some(value), res));
            }
        }
        TestResult::passed()
    }

    #[quickcheck]
    fn check_single_interval_max(array: Vec<i32>) -> TestResult {
        let seg_tree = SegmentTree::from_vec(&array, max);
        for (i, value) in array.into_iter().enumerate() {
            let res = seg_tree.query(i..(i + 1));
            if res != Some(value) {
                return TestResult::error(format!("Expected {:?}, got {:?}", Some(value), res));
            }
        }
        TestResult::passed()
    }

    #[quickcheck]
    fn check_single_interval_sum(array: Vec<i32>) -> TestResult {
        let seg_tree = SegmentTree::from_vec(&array, max);
        for (i, value) in array.into_iter().enumerate() {
            let res = seg_tree.query(i..(i + 1));
            if res != Some(value) {
                return TestResult::error(format!("Expected {:?}, got {:?}", Some(value), res));
            }
        }
        TestResult::passed()
    }
}
