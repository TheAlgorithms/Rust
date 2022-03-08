/// This stucture implements a segmented tree that
/// can efficiently answer range queries on arrays.
pub struct SegmentTree<T: Default + Ord + Copy> {
    len: usize,
    buf: Vec<T>,
    op: Ops,
}

pub enum Ops {
    Max,
    Min,
}

impl<T: Default + Ord + Copy> SegmentTree<T> {
    /// function to build the tree
    pub fn from_vec(arr: &[T], op: Ops) -> Self {
        let len = arr.len();
        let mut buf: Vec<T> = vec![T::default(); 2 * len];
        buf[len..(len + len)].clone_from_slice(&arr[0..len]);
        for i in (1..len).rev() {
            buf[i] = match op {
                Ops::Max => buf[2 * i].max(buf[2 * i + 1]),
                Ops::Min => buf[2 * i].min(buf[2 * i + 1]),
            };
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
                res = match self.op {
                    Ops::Max => res.max(self.buf[l]),
                    Ops::Min => res.min(self.buf[l]),
                };
                l += 1;
            }
            if r % 2 == 0 {
                res = match self.op {
                    Ops::Max => res.max(self.buf[r]),
                    Ops::Min => res.min(self.buf[r]),
                };
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
            self.buf[idx] = match self.op {
                Ops::Max => self.buf[2 * idx].max(self.buf[2 * idx + 1]),
                Ops::Min => self.buf[2 * idx].min(self.buf[2 * idx + 1]),
            };
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
        let min_seg_tree = SegmentTree::from_vec(&vec, Ops::Min);
        assert_eq!(-5, min_seg_tree.query(4, 6));
        assert_eq!(-20, min_seg_tree.query(0, vec.len() - 1));
        let mut max_seg_tree = SegmentTree::from_vec(&vec, Ops::Max);
        assert_eq!(6, max_seg_tree.query(4, 6));
        assert_eq!(15, max_seg_tree.query(0, vec.len() - 1));
        max_seg_tree.update(6, 8);
        assert_eq!(8, max_seg_tree.query(4, 6));
    }
}
