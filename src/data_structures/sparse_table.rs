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
    pub fn new(input: &[T]) -> SparseTable<T> {
        SparseTable {
            input: input.to_vec(),
            table: build_sparse_table(input),
        }
    }

    pub fn get_min(&self, mut l: usize, mut r: usize) -> T {
        if r < l {
            std::mem::swap(&mut r, &mut l);
        }
        let loglen = (r - l + 1).ilog2() as usize;
        let idx: usize = r + 1 - (1 << loglen);
        let a = self.table[loglen][l];
        let b = self.table[loglen][idx];
        if self.input[a] < self.input[b] {
            return self.input[a];
        }
        self.input[b]
    }
}

fn build_sparse_table<T: PartialOrd>(array: &[T]) -> Vec<Vec<usize>> {
    let mut table: Vec<Vec<usize>> = vec![(0..array.len()).collect()];
    let len = array.len();

    for loglen in 1..=len.ilog2() {
        let mut row = Vec::new();
        for i in 0..=len - (1 << loglen) {
            let a = table[table.len() - 1][i];
            let b = table[table.len() - 1][i + (1 << (loglen - 1))];
            if array[a] < array[b] {
                row.push(a);
            } else {
                row.push(b);
            }
        }
        table.push(row);
    }
    table
}

#[cfg(test)]
mod tests {
    #[test]
    fn construction_tests() {
        let v1 = [1, 3, 6, 123, 7, 235, 3, -4, 6, 2];
        let sparse_v1 = super::SparseTable::new(&v1);
        assert_eq!(
            sparse_v1.table,
            vec![
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![0, 1, 2, 4, 4, 6, 7, 7, 9],
                vec![0, 1, 2, 6, 7, 7, 7],
                vec![7, 7, 7]
            ]
        );

        let v2 = [
            20, 13, -13, 2, 3634, -2, 56, 3, 67, 8, 23, 0, -23, 1, 5, 85, 3, 24, 5, -10, 3, 4, 20,
        ];
        let sparse_v2 = super::SparseTable::new(&v2);
        assert_eq!(
            sparse_v2.table,
            vec![
                vec![
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22
                ],
                vec![1, 2, 2, 3, 5, 5, 7, 7, 9, 9, 11, 12, 12, 13, 14, 16, 16, 18, 19, 19, 20, 21],
                vec![2, 2, 2, 5, 5, 5, 7, 7, 11, 12, 12, 12, 12, 13, 16, 16, 19, 19, 19, 19],
                vec![2, 2, 2, 5, 5, 12, 12, 12, 12, 12, 12, 12, 12, 19, 19, 19],
                vec![12, 12, 12, 12, 12, 12, 12, 12]
            ]
        );
    }

    #[test]
    fn simple_query_tests() {
        let v1 = vec![1, 3, 6, 123, 7, 235, 3, -4, 6, 2];
        let sparse_v1 = super::SparseTable::new(&v1);

        assert_eq!(3, sparse_v1.get_min(1, 5));
        assert_eq!(-4, sparse_v1.get_min(0, 9));
        assert_eq!(6, sparse_v1.get_min(8, 8));
        assert_eq!(7, sparse_v1.get_min(4, 3));
    }

    #[test]
    fn float_query_tests() {
        let sparse_v1 = super::SparseTable::new(&[0.4, -2.3, 0.0, 234.22, 12.2, -3.0]);

        assert_eq!(-3.0, sparse_v1.get_min(0, 5));
        assert_eq!(-2.3, sparse_v1.get_min(0, 3));
        assert_eq!(12.2, sparse_v1.get_min(3, 4));
        assert_eq!(0.0, sparse_v1.get_min(2, 2));
    }
}
