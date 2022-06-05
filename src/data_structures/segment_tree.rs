/// This stucture implements a segmented tree that
/// can efficiently answer range queries on arrays.
pub struct SegmentTree<T: Default + Ord + Copy, F: Fn(T, T) -> T> {
    len: usize,
    buf: Vec<T>,
    op: F,
}

impl<T: Default + Ord + Copy, F: Fn(T, T) -> T> SegmentTree<T, F> {
    /// function to build the tree
    pub fn from_vec(arr: &[T], op: F) -> Self {
        let len = arr.len();
        let mut buf: Vec<T> = vec![T::default(); 2 * len];
        buf[len..(len + len)].clone_from_slice(&arr[0..len]);
        for i in (1..len).rev() {
            buf[i] = op(buf[2 * i], buf[2 * i + 1]);
        }
        SegmentTree { len, buf, op }
    }

    /// function to get sum on interval [l, r]
    pub fn query(&self, mut l: usize, mut r: usize) -> T {
        l += self.len;
        r += self.len;
        let mut res = self.buf[l];
        while l <= r {
            if l % 2 == 1 {
                res = (self.op)(res, self.buf[l]);
                l += 1;
            }
            if r % 2 == 0 {
                res = (self.op)(res, self.buf[r]);
                r -= 1;
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
            self.buf[idx] = (self.op)(self.buf[2 * idx], self.buf[2 * idx + 1]);
            idx /= 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec = vec![1, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let min_fn = |x1: i32, x2: i32| x1.min(x2);
        let max_fn = |x1: i32, x2: i32| x1.max(x2);
        let min_seg_tree = SegmentTree::from_vec(&vec, min_fn);
        assert_eq!(-5, min_seg_tree.query(4, 6));
        assert_eq!(-20, min_seg_tree.query(0, vec.len() - 1));
        let mut max_seg_tree = SegmentTree::from_vec(&vec, max_fn);
        assert_eq!(6, max_seg_tree.query(4, 6));
        assert_eq!(15, max_seg_tree.query(0, vec.len() - 1));
        max_seg_tree.update(6, 8);
        assert_eq!(8, max_seg_tree.query(4, 6));
    }
}
