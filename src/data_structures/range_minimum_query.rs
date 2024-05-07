/*
    A RangeMinimumQuery, is a data structure for answering range-minimum-queries of an array.
    For a given array A[], of elements for which an ordering exists, we want to find the
    minimum value A[x] of a subarray A[i..j], where i and j are the query parameters.

    Precomputation complexity: O(n log(n))
    Query complexity: O(1)

    Wikipedia: <https://en.wikipedia.org/wiki/Range_minimum_query>
*/

use std::cmp::PartialOrd;
use std::fmt;

/// Custom error for invalid range
#[derive(Debug, PartialEq)]
pub struct RangeError;

impl fmt::Display for RangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid range")
    }
}

pub struct RangeMinimumQuery<T: PartialOrd + Copy> {
    // the current version makes a copy of the input array, but this could be changed
    // to references if needed (in that case, we dont need T to implement the Copy trait)
    array: Vec<T>,
    sparse_table: Vec<Vec<usize>>,
}

impl<T: PartialOrd + Copy> RangeMinimumQuery<T> {
    pub fn new(input: &[T]) -> RangeMinimumQuery<T> {
        RangeMinimumQuery {
            array: input.to_vec(),
            sparse_table: build_sparse_table(input),
        }
    }

    pub fn get_range_min(&self, start: usize, end: usize) -> Result<T, RangeError> {
        if start >= end || start >= self.array.len() || end > self.array.len() {
            return Err(RangeError);
        }
        let loglen = (end - start).ilog2() as usize;
        let idx: usize = end - (1 << loglen);
        let a = self.sparse_table[loglen][start];
        let b = self.sparse_table[loglen][idx];
        if self.array[a] < self.array[b] {
            return Ok(self.array[a]);
        }
        Ok(self.array[b])
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
    use super::build_sparse_table;
    macro_rules! test_build_sparse_table {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (array, expected) = $inputs;
                assert_eq!(build_sparse_table(&array), expected);
            }
        )*
        }
    }
    test_build_sparse_table! {
    small: ([1, 6, 3], vec![vec![0, 1, 2], vec![0, 2]]),
    tc_1: ([1, 3, 6, 123, 7, 235, 3, -4, 6, 2], vec![
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![0, 1, 2, 4, 4, 6, 7, 7, 9],
        vec![0, 1, 2, 6, 7, 7, 7],
        vec![7, 7, 7]
    ]),
    tc_2: ([
        20, 13, -13, 2, 3634, -2, 56, 3, 67, 8, 23, 0, -23, 1, 5, 85, 3, 24, 5, -10, 3, 4, 20,
    ], vec![
        vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
            22
        ],
        vec![1, 2, 2, 3, 5, 5, 7, 7, 9, 9, 11, 12, 12, 13, 14, 16, 16, 18, 19, 19, 20, 21],
        vec![2, 2, 2, 5, 5, 5, 7, 7, 11, 12, 12, 12, 12, 13, 16, 16, 19, 19, 19, 19],
        vec![2, 2, 2, 5, 5, 12, 12, 12, 12, 12, 12, 12, 12, 19, 19, 19],
        vec![12, 12, 12, 12, 12, 12, 12, 12]
    ]),
    }

    #[test]
    fn simple_query_tests() {
        let v1 = vec![1, 3, 6, 123, 7, 235, 3, -4, 6, 2];
        let sparse_v1 = super::RangeMinimumQuery::new(&v1);

        assert_eq!(Ok(3), sparse_v1.get_range_min(1, 6));
        assert_eq!(Ok(-4), sparse_v1.get_range_min(0, 10));
        assert_eq!(Ok(6), sparse_v1.get_range_min(8, 9));
        assert!(sparse_v1.get_range_min(4, 3).is_err());
        assert!(sparse_v1.get_range_min(0, 1000).is_err());
        assert!(sparse_v1.get_range_min(1000, 1001).is_err());
    }

    #[test]
    fn float_query_tests() {
        let sparse_v1 = super::RangeMinimumQuery::new(&[0.4, -2.3, 0.0, 234.22, 12.2, -3.0]);

        assert_eq!(Ok(-3.0), sparse_v1.get_range_min(0, 6));
        assert_eq!(Ok(-2.3), sparse_v1.get_range_min(0, 4));
        assert_eq!(Ok(12.2), sparse_v1.get_range_min(3, 5));
        assert_eq!(Ok(0.0), sparse_v1.get_range_min(2, 3));
    }
}
