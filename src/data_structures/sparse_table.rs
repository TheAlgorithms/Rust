/*
    A Sparse Table, is a data structure for answering range-minimum-queries of an array.
    For a given array A[], of elements for which an ordering exists, we want to find the
    minimum value A[x] of a subarray A[i..j], where i and j are the query parameters.

    Precomputation complexity: O(n log(n))
    Query complexity: O(1)

    Wikipedia: <https://en.wikipedia.org/wiki/Range_minimum_query>
*/

use std::cmp::PartialOrd;

pub struct SparseTable<T: PartialOrd + Copy> {
    // the current version makes a copy of the input array, but this could be changed
    // to references if needed (in that case, we dont need T to implement the Copy trait)
    input: Vec<T>,
    table: Vec<Vec<usize>>,
}

impl<T: PartialOrd + Copy> SparseTable<T> {
    pub fn new(input: &Vec<T>) -> SparseTable<T> {
        let mut table: Vec<Vec<usize>> = vec![(0..input.len()).collect()];
        let len = input.len();

        for loglen in 1..=len.ilog2() {
            let mut row = Vec::new();
            for i in 0..=len - (1 << loglen) {
                let a = table[table.len() - 1][i];
                let b = table[table.len() - 1][i + (1 << (loglen - 1))];
                if input[a] < input[b] {
                    row.push(a);
                } else {
                    row.push(b);
                }
            }
            table.push(row);
        }
        SparseTable {
            input: input.clone(),
            table,
        }
    }

    pub fn get_min(&self, mut l: usize, mut r: usize) -> T {
        if r < l {
            std::mem::swap(&mut r, &mut l);
        }
        let loglen = (r - l + 1).ilog2() as usize;
        let idx: usize = r - (1 << loglen) + 1;
        let a = self.table[loglen][l];
        let b = self.table[loglen][idx];
        if self.input[a] < self.input[b] {
            return self.input[a];
        }
        self.input[b]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_tests() {
        let v1 = vec![1, 3, 6, 123, 7, 235, 3, -4, 6, 2];
        let sparse_v1 = super::SparseTable::new(&v1);

        assert_eq!(3, sparse_v1.get_min(1, 5));
        assert_eq!(-4, sparse_v1.get_min(0, 9));
        assert_eq!(6, sparse_v1.get_min(8, 8));
        assert_eq!(7, sparse_v1.get_min(4, 3));
    }
}
