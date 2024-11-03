use std::fmt::Debug;
use std::ops::Range;

/// Custom error types representing possible errors that can occur during operations on the `SegmentTree`.
#[derive(Debug, PartialEq, Eq)]
pub enum SegmentTreeError {
    /// Error indicating that an index is out of bounds.
    IndexOutOfBounds,
    /// Error indicating that a range provided for a query is invalid.
    InvalidRange,
}

/// A data structure representing a Segment Tree. Which is used for efficient
/// range queries and updates on an array of elements.
pub struct SegmentTree<T, F>
where
    T: Debug + Default + Ord + Copy,
    F: Fn(T, T) -> T,
{
    /// The number of elements in the original input array for which the segment tree is built.
    size: usize,
    /// A vector representing the nodes of the segment tree.
    nodes: Vec<T>,
    /// A function that merges two elements of type `T`.
    merge_fn: F,
}

impl<T, F> SegmentTree<T, F>
where
    T: Debug + Default + Ord + Copy,
    F: Fn(T, T) -> T,
{
    /// Creates a new `SegmentTree` from the provided slice of elements.
    ///
    /// # Arguments
    ///
    /// * `arr`: A slice of elements of type `T` that initializes the segment tree.
    /// * `merge_fn`: A merging function that specifies how to combine two elements of type `T`.
    ///
    /// # Returns
    ///
    /// A new `SegmentTree` instance initialized with the given elements.
    pub fn from_vec(arr: &[T], merge_fn: F) -> Self {
        let size = arr.len();
        let mut seg_tree = SegmentTree {
            size,
            nodes: vec![T::default(); 4 * size],
            merge_fn,
        };
        if size != 0 {
            seg_tree.build_recursive(arr, 1, 0..size);
        }
        seg_tree
    }

    /// Recursively builds the segment tree from the provided array.
    ///
    /// # Parameters
    ///
    /// * `arr` - The original array of values.
    /// * `node_idx` - The index of the current node in the segment tree.
    /// * `node_range` - The range of elements in the original array that the current node covers.
    fn build_recursive(&mut self, arr: &[T], node_idx: usize, node_range: Range<usize>) {
        if node_range.end - node_range.start == 1 {
            self.nodes[node_idx] = arr[node_range.start];
        } else {
            let mid = node_range.start + (node_range.end - node_range.start) / 2;
            self.build_recursive(arr, 2 * node_idx, node_range.start..mid);
            self.build_recursive(arr, 2 * node_idx + 1, mid..node_range.end);
            self.nodes[node_idx] =
                (self.merge_fn)(self.nodes[2 * node_idx], self.nodes[2 * node_idx + 1]);
        }
    }

    /// Queries the segment tree for the result of merging the elements in the specified range.
    ///
    /// # Arguments
    ///
    /// * `target_range`: A range specified as `Range<usize>`, indicating the start (inclusive)
    ///   and end (exclusive) indices of the segment to query.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(result))` if the query is successful and there are elements in the range,
    /// * `Ok(None)` if the range is empty,
    /// * `Err(SegmentTreeError::InvalidRange)` if the provided range is invalid.
    pub fn query(&self, target_range: Range<usize>) -> Result<Option<T>, SegmentTreeError> {
        if target_range.start >= self.size || target_range.end > self.size {
            return Err(SegmentTreeError::InvalidRange);
        }
        Ok(self.query_recursive(1, 0..self.size, &target_range))
    }

    /// Recursively performs a range query to find the merged result of the specified range.
    ///
    /// # Parameters
    ///
    /// * `node_idx` - The index of the current node in the segment tree.
    /// * `tree_range` - The range of elements covered by the current node.
    /// * `target_range` - The range for which the query is being performed.
    ///
    /// # Returns
    ///
    /// An `Option<T>` containing the result of the merge operation on the range if within bounds,
    /// or `None` if the range is outside the covered range.
    fn query_recursive(
        &self,
        node_idx: usize,
        tree_range: Range<usize>,
        target_range: &Range<usize>,
    ) -> Option<T> {
        if tree_range.start >= target_range.end || tree_range.end <= target_range.start {
            return None;
        }
        if tree_range.start >= target_range.start && tree_range.end <= target_range.end {
            return Some(self.nodes[node_idx]);
        }
        let mid = tree_range.start + (tree_range.end - tree_range.start) / 2;
        let left_res = self.query_recursive(node_idx * 2, tree_range.start..mid, target_range);
        let right_res = self.query_recursive(node_idx * 2 + 1, mid..tree_range.end, target_range);
        match (left_res, right_res) {
            (None, None) => None,
            (None, Some(r)) => Some(r),
            (Some(l), None) => Some(l),
            (Some(l), Some(r)) => Some((self.merge_fn)(l, r)),
        }
    }

    /// Updates the value at the specified index in the segment tree.
    ///
    /// # Arguments
    ///
    /// * `target_idx`: The index (0-based) of the element to update.
    /// * `val`: The new value of type `T` to set at the specified index.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the update was successful,
    /// * `Err(SegmentTreeError::IndexOutOfBounds)` if the index is out of bounds.
    pub fn update(&mut self, target_idx: usize, val: T) -> Result<(), SegmentTreeError> {
        if target_idx >= self.size {
            return Err(SegmentTreeError::IndexOutOfBounds);
        }
        self.update_recursive(1, 0..self.size, target_idx, val);
        Ok(())
    }

    /// Recursively updates the segment tree for a specific index with a new value.
    ///
    /// # Parameters
    ///
    /// * `node_idx` - The index of the current node in the segment tree.
    /// * `tree_range` - The range of elements covered by the current node.
    /// * `target_idx` - The index in the original array to update.
    /// * `val` - The new value to set at `target_idx`.
    fn update_recursive(
        &mut self,
        node_idx: usize,
        tree_range: Range<usize>,
        target_idx: usize,
        val: T,
    ) {
        if tree_range.start > target_idx || tree_range.end <= target_idx {
            return;
        }
        if tree_range.end - tree_range.start <= 1 && tree_range.start == target_idx {
            self.nodes[node_idx] = val;
            return;
        }
        let mid = tree_range.start + (tree_range.end - tree_range.start) / 2;
        self.update_recursive(node_idx * 2, tree_range.start..mid, target_idx, val);
        self.update_recursive(node_idx * 2 + 1, mid..tree_range.end, target_idx, val);
        self.nodes[node_idx] =
            (self.merge_fn)(self.nodes[node_idx * 2], self.nodes[node_idx * 2 + 1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::{max, min};

    #[test]
    fn test_min_segments() {
        let vec = vec![-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut min_seg_tree = SegmentTree::from_vec(&vec, min);
        assert_eq!(min_seg_tree.query(4..7), Ok(Some(-5)));
        assert_eq!(min_seg_tree.query(0..vec.len()), Ok(Some(-30)));
        assert_eq!(min_seg_tree.query(0..2), Ok(Some(-30)));
        assert_eq!(min_seg_tree.query(1..3), Ok(Some(-4)));
        assert_eq!(min_seg_tree.query(1..7), Ok(Some(-5)));
        assert_eq!(min_seg_tree.update(5, 10), Ok(()));
        assert_eq!(min_seg_tree.update(14, -8), Ok(()));
        assert_eq!(min_seg_tree.query(4..7), Ok(Some(3)));
        assert_eq!(
            min_seg_tree.update(15, 100),
            Err(SegmentTreeError::IndexOutOfBounds)
        );
        assert_eq!(min_seg_tree.query(5..5), Ok(None));
        assert_eq!(
            min_seg_tree.query(10..16),
            Err(SegmentTreeError::InvalidRange)
        );
        assert_eq!(
            min_seg_tree.query(15..20),
            Err(SegmentTreeError::InvalidRange)
        );
    }

    #[test]
    fn test_max_segments() {
        let vec = vec![1, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut max_seg_tree = SegmentTree::from_vec(&vec, max);
        assert_eq!(max_seg_tree.query(0..vec.len()), Ok(Some(15)));
        assert_eq!(max_seg_tree.query(3..5), Ok(Some(7)));
        assert_eq!(max_seg_tree.query(4..8), Ok(Some(11)));
        assert_eq!(max_seg_tree.query(8..10), Ok(Some(9)));
        assert_eq!(max_seg_tree.query(9..12), Ok(Some(15)));
        assert_eq!(max_seg_tree.update(4, 10), Ok(()));
        assert_eq!(max_seg_tree.update(14, -8), Ok(()));
        assert_eq!(max_seg_tree.query(3..5), Ok(Some(10)));
        assert_eq!(
            max_seg_tree.update(15, 100),
            Err(SegmentTreeError::IndexOutOfBounds)
        );
        assert_eq!(max_seg_tree.query(5..5), Ok(None));
        assert_eq!(
            max_seg_tree.query(10..16),
            Err(SegmentTreeError::InvalidRange)
        );
        assert_eq!(
            max_seg_tree.query(15..20),
            Err(SegmentTreeError::InvalidRange)
        );
    }

    #[test]
    fn test_sum_segments() {
        let vec = vec![1, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut sum_seg_tree = SegmentTree::from_vec(&vec, |a, b| a + b);
        assert_eq!(sum_seg_tree.query(0..vec.len()), Ok(Some(38)));
        assert_eq!(sum_seg_tree.query(1..4), Ok(Some(5)));
        assert_eq!(sum_seg_tree.query(4..7), Ok(Some(4)));
        assert_eq!(sum_seg_tree.query(6..9), Ok(Some(-3)));
        assert_eq!(sum_seg_tree.query(9..vec.len()), Ok(Some(37)));
        assert_eq!(sum_seg_tree.update(5, 10), Ok(()));
        assert_eq!(sum_seg_tree.update(14, -8), Ok(()));
        assert_eq!(sum_seg_tree.query(4..7), Ok(Some(19)));
        assert_eq!(
            sum_seg_tree.update(15, 100),
            Err(SegmentTreeError::IndexOutOfBounds)
        );
        assert_eq!(sum_seg_tree.query(5..5), Ok(None));
        assert_eq!(
            sum_seg_tree.query(10..16),
            Err(SegmentTreeError::InvalidRange)
        );
        assert_eq!(
            sum_seg_tree.query(15..20),
            Err(SegmentTreeError::InvalidRange)
        );
    }
}
