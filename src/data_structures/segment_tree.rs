use std::cmp::min;
use std::fmt::Debug;
use std::ops::Range;

/// This stucture implements a segmented tree that
/// can efficiently answer range queries on arrays.

/// We need a reduction function for each segment or interval. It could be the min over this interval, the max, the sum, etc.

pub struct SegmentTree<T: Debug + Default + Ord + Copy> {
    len: usize,
    buf: Vec<T>,
    merge: fn(T, T) -> T,
}

impl<T: Debug + Default + Ord + Copy> SegmentTree<T> {
    /// function to build the tree
    pub fn from_vec(arr: &[T], merge: fn(T, T) -> T) -> Self {
        let len = arr.len();
        let mut buf: Vec<T> = vec![T::default(); 2 * len];
        buf[len..(len + len)].clone_from_slice(&arr[0..len]);
        for i in (1..len).rev() {
            let old = buf[2 * i];
            let new = buf[2 * i + 1];
            buf[i] = merge(old, new);
        }
        SegmentTree { len, buf, merge }
    }

    /// query the range, will return None if the range is out of the array's boundaries
    pub fn query(&self, range: Range<usize>) -> Option<T> {
        let mut l = range.start + self.len;
        let mut r = min(self.len, range.end) + self.len;
        let mut res = None;
        while l < r {
            if l % 2 == 1 {
                res = Some(match res {
                    None => self.buf[l],
                    Some(old) => (self.merge)(old, self.buf[l]),
                });
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                res = Some(match res {
                    None => self.buf[r],
                    Some(old) => (self.merge)(old, self.buf[r]),
                });
            }
            l /= 2;
            r /= 2;
        }
        res
    }

    /// function to update a tree node
    pub fn update(&mut self, mut idx: usize, val: T) {
        idx += self.len;
        self.buf[idx] = val;
        idx /= 2;

        while idx != 0 {
            self.buf[idx] = (self.merge)(self.buf[2 * idx], self.buf[2 * idx + 1]);
            idx /= 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        for (i, val) in vec.iter().enumerate() {
            assert_eq!(Some(*val), min_seg_tree.query(i..(i + 1)));
        }
    }

    #[test]
    fn test_max_segments() {
        let vec = vec![1, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut max_seg_tree = SegmentTree::from_vec(&vec, max);
        assert_eq!(Some(15), max_seg_tree.query(0..vec.len()));
        assert_eq!(Some(6), max_seg_tree.query(4..7));
        for (i, val) in vec.iter().enumerate() {
            assert_eq!(Some(*val), max_seg_tree.query(i..(i + 1)));
        }
        max_seg_tree.update(6, 8);
        assert_eq!(Some(8), max_seg_tree.query(4..7));
    }

    #[test]
    fn test_sum_segments() {
        let val_at_6 = 6;
        let vec = vec![1, 2, -4, 7, 3, -5, val_at_6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut sum_seg_tree = SegmentTree::from_vec(&vec, |a, b| a + b);
        assert_eq!(
            Some(vec.iter().sum::<i32>()),
            sum_seg_tree.query(0..vec.len())
        );
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
}
