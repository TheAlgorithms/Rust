use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Range};

pub struct LazySegmentTree<T: Debug + Default + Ord + Copy + Display + AddAssign + Add<Output = T>>
{
    len: usize,
    tree: Vec<T>,
    lazy: Vec<Option<T>>,
    merge: fn(T, T) -> T,
    /// apply: fn(T, usize, T) -> T takes
    /// 1. the node's current cached aggregate
    /// 2. the number of elements that node's range covers
    /// 3. the pending per-element delta
    ///
    /// Returns what the aggregate should become after that delta is applied to every element
    /// in the range. The range length is there because the correct answer depends on it:
    /// - for min/max the aggregate just shifts by val regardless of length
    /// - for sum it shifts by val * len.
    ///
    /// It's stored as a plain function pointer alongside merge so each node can bring its
    /// own cached value up to date in O(1), without recursing into its children to recompute
    /// it from scratch.
    ///
    apply: fn(T, usize, T) -> T,
}

impl<T: Debug + Default + Ord + Copy + Display + AddAssign + Add<Output = T>> LazySegmentTree<T> {
    pub fn from_vec(arr: &[T], merge: fn(T, T) -> T, apply: fn(T, usize, T) -> T) -> Self {
        let len = arr.len();
        let mut sgtr = LazySegmentTree {
            len,
            tree: vec![T::default(); 4 * len],
            lazy: vec![None; 4 * len],
            merge,
            apply,
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

    pub fn query(&mut self, range: Range<usize>) -> Option<T> {
        self.query_recursive(1, 0..self.len, &range)
    }

    fn query_recursive(
        &mut self,
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
        self.propagate(idx, &element_range);
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

    pub fn update(&mut self, target_range: Range<usize>, val: T) {
        self.update_recursive(1, 0..self.len, &target_range, val);
    }

    fn update_recursive(
        &mut self,
        idx: usize,
        element_range: Range<usize>,
        target_range: &Range<usize>,
        val: T,
    ) {
        if element_range.start >= target_range.end || element_range.end <= target_range.start {
            return;
        }
        if element_range.start >= target_range.start && element_range.end <= target_range.end {
            self.apply_node(idx, element_range.end - element_range.start, val);
            return;
        }
        self.propagate(idx, &element_range);
        let mid = element_range.start + (element_range.end - element_range.start) / 2;
        self.update_recursive(idx * 2, element_range.start..mid, target_range, val);
        self.update_recursive(idx * 2 + 1, mid..element_range.end, target_range, val);
        self.tree[idx] = (self.merge)(self.tree[idx * 2], self.tree[idx * 2 + 1]);
    }

    fn apply_node(&mut self, idx: usize, len: usize, val: T) {
        self.tree[idx] = (self.apply)(self.tree[idx], len, val);
        self.lazy[idx] = match self.lazy[idx] {
            Some(lazy) => Some(lazy + val),
            None => Some(val),
        };
    }

    fn propagate(&mut self, idx: usize, element_range: &Range<usize>) {
        if let Some(lazy) = self.lazy[idx].take() {
            let mid = element_range.start + (element_range.end - element_range.start) / 2;
            self.apply_node(idx * 2, mid - element_range.start, lazy);
            self.apply_node(idx * 2 + 1, element_range.end - mid, lazy);
        }
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
        // min is shift-invariant: adding `val` to every element in a range shifts
        // the min by exactly `val`, regardless of how many elements are in it.
        let mut min_seg_tree = LazySegmentTree::from_vec(&vec, min, |x, _len, val| x + val);
        // [-30, 2, -4, 7, (3, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-5), min_seg_tree.query(4..7));
        // [(-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8)]
        assert_eq!(Some(-30), min_seg_tree.query(0..vec.len()));
        // [(-30, 2), -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-30), min_seg_tree.query(0..2));
        // [-30, (2, -4), 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-4), min_seg_tree.query(1..3));
        // [-30, (2, -4, 7, 3, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-5), min_seg_tree.query(1..7));
    }

    #[test]
    fn test_max_segments() {
        let vec = vec![-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        // Same as min: max is shift-invariant, so `len` is unused here.
        let mut max_seg_tree = LazySegmentTree::from_vec(&vec, max, |x, _len, val| x + val);
        // [-30, 2, -4, 7, (3, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(6), max_seg_tree.query(4..7));
        // [(-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8)]
        assert_eq!(Some(15), max_seg_tree.query(0..vec.len()));
        // [(-30, 2), -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(2), max_seg_tree.query(0..2));
        // [-30, (2, -4), 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(2), max_seg_tree.query(1..3));
        // [-30, (2, -4, 7, 3, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(7), max_seg_tree.query(1..7));
    }

    #[test]
    fn test_sum_segments() {
        let vec = vec![-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        // sum is NOT shift-invariant: adding `val` to every element in a `len`-sized
        // range raises the sum by `val * len`, not just `val`.
        let mut max_seg_tree =
            LazySegmentTree::from_vec(&vec, |x, y| x + y, |x, len, val| x + val * len as i32);
        // [-30, 2, -4, 7, (3, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(4), max_seg_tree.query(4..7));
        // [(-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8)]
        assert_eq!(Some(7), max_seg_tree.query(0..vec.len()));
        // [(-30, 2), -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-28), max_seg_tree.query(0..2));
        // [-30, (2, -4), 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-2), max_seg_tree.query(1..3));
        // [-30, (2, -4, 7, 3, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(9), max_seg_tree.query(1..7));
    }

    #[test]
    fn test_update_segments_tiny() {
        let vec = vec![0, 0, 0, 0, 0];
        let mut update_seg_tree =
            LazySegmentTree::from_vec(&vec, |x, y| x + y, |x, len, val| x + val * len as i32);
        update_seg_tree.update(0..3, 3);
        update_seg_tree.update(2..5, 3);
        assert_eq!(Some(3), update_seg_tree.query(0..1));
        assert_eq!(Some(3), update_seg_tree.query(1..2));
        assert_eq!(Some(6), update_seg_tree.query(2..3));
        assert_eq!(Some(3), update_seg_tree.query(3..4));
        assert_eq!(Some(3), update_seg_tree.query(4..5));
    }

    #[test]
    fn test_update_segments() {
        let vec = vec![-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut update_seg_tree =
            LazySegmentTree::from_vec(&vec, |x, y| x + y, |x, len, val| x + val * len as i32);
        // -> [-30, (5, -1, 10, 6), -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        update_seg_tree.update(1..5, 3);

        // [-30, 5, -1, 10, (6 -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(7), update_seg_tree.query(4..7));
        // [(-30, 5, -1, 10, 6 , -5, 6, 11, -20, 9, 14, 15, 5, 2, -8)]
        assert_eq!(Some(19), update_seg_tree.query(0..vec.len()));
        // [(-30, 5), -1, 10, 6, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-25), update_seg_tree.query(0..2));
        // [-30, (5, -1), 10, 6, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(4), update_seg_tree.query(1..3));
        // [-30, (5, -1, 10, 6, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(21), update_seg_tree.query(1..7));
    }

    // Some properties over segment trees:
    //  When asking for the range of the overall array, return the same as iter().min() or iter().max(), etc.
    //  When asking for an interval containing a single value, return this value, no matter the merge function

    #[quickcheck]
    fn check_overall_interval_min(array: Vec<i32>) -> TestResult {
        let mut seg_tree = LazySegmentTree::from_vec(&array, min, |x, _len, val| x + val);
        TestResult::from_bool(array.iter().min().copied() == seg_tree.query(0..array.len()))
    }

    #[quickcheck]
    fn check_overall_interval_max(array: Vec<i32>) -> TestResult {
        let mut seg_tree = LazySegmentTree::from_vec(&array, max, |x, _len, val| x + val);
        TestResult::from_bool(array.iter().max().copied() == seg_tree.query(0..array.len()))
    }

    #[quickcheck]
    fn check_overall_interval_sum(array: Vec<i32>) -> TestResult {
        let mut seg_tree = LazySegmentTree::from_vec(&array, max, |x, _len, val| x + val);
        TestResult::from_bool(array.iter().max().copied() == seg_tree.query(0..array.len()))
    }

    #[quickcheck]
    fn check_single_interval_min(array: Vec<i32>) -> TestResult {
        let mut seg_tree = LazySegmentTree::from_vec(&array, min, |x, _len, val| x + val);
        for (i, value) in array.into_iter().enumerate() {
            let res = seg_tree.query(Range {
                start: i,
                end: i + 1,
            });
            if res != Some(value) {
                return TestResult::error(format!("Expected {:?}, got {:?}", Some(value), res));
            }
        }
        TestResult::passed()
    }

    #[quickcheck]
    fn check_single_interval_max(array: Vec<i32>) -> TestResult {
        let mut seg_tree = LazySegmentTree::from_vec(&array, max, |x, _len, val| x + val);
        for (i, value) in array.into_iter().enumerate() {
            let res = seg_tree.query(Range {
                start: i,
                end: i + 1,
            });
            if res != Some(value) {
                return TestResult::error(format!("Expected {:?}, got {:?}", Some(value), res));
            }
        }
        TestResult::passed()
    }

    #[quickcheck]
    fn check_single_interval_sum(array: Vec<i32>) -> TestResult {
        let mut seg_tree = LazySegmentTree::from_vec(&array, max, |x, _len, val| x + val);
        for (i, value) in array.into_iter().enumerate() {
            let res = seg_tree.query(Range {
                start: i,
                end: i + 1,
            });
            if res != Some(value) {
                return TestResult::error(format!("Expected {:?}, got {:?}", Some(value), res));
            }
        }
        TestResult::passed()
    }

    #[test]
    fn test_large_array_min() {
        let n: usize = 1 << 20;
        let arr = vec![0i64; n];
        let mut tree = LazySegmentTree::from_vec(&arr, min, |x, _len, val| x + val);
        for i in 0..1000 {
            tree.update(i..i + 1, 1);
        }
        assert_eq!(Some(1), tree.query(0..1000));
        assert_eq!(Some(0), tree.query(0..n));
    }

    #[test]
    fn test_large_array_max() {
        let n: usize = 1 << 20;
        let arr = vec![0i64; n];
        let mut tree = LazySegmentTree::from_vec(&arr, max, |x, _len, val| x + val);
        for i in 0..1000 {
            tree.update(i..i + 1, 1);
        }
        assert_eq!(Some(1), tree.query(0..1000));
        assert_eq!(Some(1), tree.query(0..n));
    }

    #[test]
    fn test_large_array_sum() {
        let n: usize = 1 << 20;
        let arr = vec![0i64; n];
        let mut tree =
            LazySegmentTree::from_vec(&arr, |x, y| x + y, |x, len, val| x + val * len as i64);
        for i in 0..1000 {
            tree.update(i..i + 1, 1);
        }
        assert_eq!(Some(1000), tree.query(0..1000));
        assert_eq!(Some(0), tree.query(1000..n));
    }
}
