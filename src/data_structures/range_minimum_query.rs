//! Range Minimum Query (RMQ) Implementation
//!
//! This module provides an efficient implementation of a Range Minimum Query data structure using a
//! sparse table approach. It allows for quick retrieval of the minimum value within a specified subdata
//! of a given data after an initial preprocessing phase.
//!
//! The RMQ is particularly useful in scenarios requiring multiple queries on static data, as it
//! allows querying in constant time after an O(n log(n)) preprocessing time.
//!
//! References: [Wikipedia](https://en.wikipedia.org/wiki/Range_minimum_query)

use std::cmp::PartialOrd;

/// Custom error type for invalid range queries.
#[derive(Debug, PartialEq, Eq)]
pub enum RangeError {
    /// Indicates that the provided range is invalid (start index is not less than end index).
    InvalidRange,
    /// Indicates that one or more indices are out of bounds for the data.
    IndexOutOfBound,
}

/// A data structure for efficiently answering range minimum queries on static data.
pub struct RangeMinimumQuery<T: PartialOrd + Copy> {
    /// The original input data on which range queries are performed.
    data: Vec<T>,
    /// The sparse table for storing preprocessed range minimum information. Each entry
    /// contains the index of the minimum element in the range starting at `j` and having a length of `2^i`.
    sparse_table: Vec<Vec<usize>>,
}

impl<T: PartialOrd + Copy> RangeMinimumQuery<T> {
    /// Creates a new `RangeMinimumQuery` instance with the provided input data.
    ///
    /// # Arguments
    ///
    /// * `input` - A slice of elements of type `T` that implement `PartialOrd` and `Copy`.
    ///
    /// # Returns
    ///
    /// A `RangeMinimumQuery` instance that can be used to perform range minimum queries.
    pub fn new(input: &[T]) -> RangeMinimumQuery<T> {
        RangeMinimumQuery {
            data: input.to_vec(),
            sparse_table: build_sparse_table(input),
        }
    }

    /// Retrieves the minimum value in the specified range [start, end).
    ///
    /// # Arguments
    ///
    /// * `start` - The starting index of the range (inclusive).
    /// * `end` - The ending index of the range (exclusive).
    ///
    /// # Returns
    ///
    /// * `Ok(T)` - The minimum value found in the specified range.
    /// * `Err(RangeError)` - An error indicating the reason for failure, such as an invalid range
    ///   or indices out of bounds.
    pub fn get_range_min(&self, start: usize, end: usize) -> Result<T, RangeError> {
        // Validate range
        if start >= end {
            return Err(RangeError::InvalidRange);
        }
        if start >= self.data.len() || end > self.data.len() {
            return Err(RangeError::IndexOutOfBound);
        }

        // Calculate the log length and the index for the sparse table
        let log_len = (end - start).ilog2() as usize;
        let idx: usize = end - (1 << log_len);

        // Retrieve the indices of the minimum values from the sparse table
        let min_idx_start = self.sparse_table[log_len][start];
        let min_idx_end = self.sparse_table[log_len][idx];

        // Compare the values at the retrieved indices and return the minimum
        if self.data[min_idx_start] < self.data[min_idx_end] {
            Ok(self.data[min_idx_start])
        } else {
            Ok(self.data[min_idx_end])
        }
    }
}

/// Builds a sparse table for the provided data to support range minimum queries.
///
/// # Arguments
///
/// * `data` - A slice of elements of type `T` that implement `PartialOrd`.
///
/// # Returns
///
/// A 2D vector representing the sparse table, where each entry contains the index of the minimum
/// element in the range defined by the starting index and the power of two lengths.
fn build_sparse_table<T: PartialOrd>(data: &[T]) -> Vec<Vec<usize>> {
    let mut sparse_table: Vec<Vec<usize>> = vec![(0..data.len()).collect()];
    let len = data.len();

    // Fill the sparse table
    for log_len in 1..=len.ilog2() {
        let mut row = Vec::new();
        for idx in 0..=len - (1 << log_len) {
            let min_idx_start = sparse_table[sparse_table.len() - 1][idx];
            let min_idx_end = sparse_table[sparse_table.len() - 1][idx + (1 << (log_len - 1))];
            if data[min_idx_start] < data[min_idx_end] {
                row.push(min_idx_start);
            } else {
                row.push(min_idx_end);
            }
        }
        sparse_table.push(row);
    }

    sparse_table
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_build_sparse_table {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (data, expected) = $inputs;
                    assert_eq!(build_sparse_table(&data), expected);
                }
            )*
        }
    }

    test_build_sparse_table! {
        small: (
            [1, 6, 3],
            vec![
                vec![0, 1, 2],
                vec![0, 2]
            ]
        ),
        medium: (
            [1, 3, 6, 123, 7, 235, 3, -4, 6, 2],
            vec![
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![0, 1, 2, 4, 4, 6, 7, 7, 9],
                vec![0, 1, 2, 6, 7, 7, 7],
                vec![7, 7, 7]
            ]
        ),
        large: (
            [20, 13, -13, 2, 3634, -2, 56, 3, 67, 8, 23, 0, -23, 1, 5, 85, 3, 24, 5, -10, 3, 4, 20],
            vec![
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22],
                vec![1, 2, 2, 3, 5, 5, 7, 7, 9, 9, 11, 12, 12, 13, 14, 16, 16, 18, 19, 19, 20, 21],
                vec![2, 2, 2, 5, 5, 5, 7, 7, 11, 12, 12, 12, 12, 13, 16, 16, 19, 19, 19, 19],
                vec![2, 2, 2, 5, 5, 12, 12, 12, 12, 12, 12, 12, 12, 19, 19, 19],
                vec![12, 12, 12, 12, 12, 12, 12, 12]
            ]
        ),
    }

    #[test]
    fn simple_query_tests() {
        let rmq = RangeMinimumQuery::new(&[1, 3, 6, 123, 7, 235, 3, -4, 6, 2]);

        assert_eq!(rmq.get_range_min(1, 6), Ok(3));
        assert_eq!(rmq.get_range_min(0, 10), Ok(-4));
        assert_eq!(rmq.get_range_min(8, 9), Ok(6));
        assert_eq!(rmq.get_range_min(4, 3), Err(RangeError::InvalidRange));
        assert_eq!(rmq.get_range_min(0, 1000), Err(RangeError::IndexOutOfBound));
        assert_eq!(
            rmq.get_range_min(1000, 1001),
            Err(RangeError::IndexOutOfBound)
        );
    }

    #[test]
    fn float_query_tests() {
        let rmq = RangeMinimumQuery::new(&[0.4, -2.3, 0.0, 234.22, 12.2, -3.0]);

        assert_eq!(rmq.get_range_min(0, 6), Ok(-3.0));
        assert_eq!(rmq.get_range_min(0, 4), Ok(-2.3));
        assert_eq!(rmq.get_range_min(3, 5), Ok(12.2));
        assert_eq!(rmq.get_range_min(2, 3), Ok(0.0));
        assert_eq!(rmq.get_range_min(4, 3), Err(RangeError::InvalidRange));
        assert_eq!(rmq.get_range_min(0, 1000), Err(RangeError::IndexOutOfBound));
        assert_eq!(
            rmq.get_range_min(1000, 1001),
            Err(RangeError::IndexOutOfBound)
        );
    }
}
