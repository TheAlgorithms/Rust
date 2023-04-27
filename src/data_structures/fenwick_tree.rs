use std::ops::{Add, AddAssign};

/// Fenwick Tree / Binary Indexed Tree
///
/// Consider we have an array `arr[0...n-1]`. We would like to:
/// 1. Compute the sum of the first i elements.
/// 2. Modify the value of a specified element of the array `arr[i] = x`, where `0 <= i <= n-1`.
pub struct FenwickTree<T: Add + AddAssign + Copy + Default> {
    data: Vec<T>,
}

impl<T: Add<Output = T> + AddAssign + Copy + Default> FenwickTree<T> {
    /// construct a new FenwickTree with given length
    pub fn with_len(len: usize) -> Self {
        FenwickTree {
            data: vec![T::default(); len + 1],
        }
    }

    /// add `val` to `idx`
    pub fn add(&mut self, i: usize, val: T) {
        assert!(i < self.data.len());
        let mut i = i + 1;
        while i < self.data.len() {
            self.data[i] += val;
            i += lowbit(i);
        }
    }

    /// get the sum of [0, i]
    pub fn prefix_sum(&self, i: usize) -> T {
        assert!(i < self.data.len());
        let mut i = i + 1;
        let mut res = T::default();
        while i > 0 {
            res += self.data[i];
            i -= lowbit(i);
        }
        res
    }
}

/// get the lowest bit of `i`
const fn lowbit(x: usize) -> usize {
    let x = x as isize;
    (x & (-x)) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut ft = FenwickTree::with_len(10);
        ft.add(0, 1);
        ft.add(1, 2);
        ft.add(2, 3);
        ft.add(3, 4);
        ft.add(4, 5);
        ft.add(5, 6);
        ft.add(6, 7);
        ft.add(7, 8);
        ft.add(8, 9);
        ft.add(9, 10);
        assert_eq!(ft.prefix_sum(0), 1);
        assert_eq!(ft.prefix_sum(1), 3);
        assert_eq!(ft.prefix_sum(2), 6);
        assert_eq!(ft.prefix_sum(3), 10);
        assert_eq!(ft.prefix_sum(4), 15);
        assert_eq!(ft.prefix_sum(5), 21);
        assert_eq!(ft.prefix_sum(6), 28);
        assert_eq!(ft.prefix_sum(7), 36);
        assert_eq!(ft.prefix_sum(8), 45);
        assert_eq!(ft.prefix_sum(9), 55);
    }
}
