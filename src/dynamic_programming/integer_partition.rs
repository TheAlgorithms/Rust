//! Integer partition using dynamic programming
//!
//! The number of partitions of a number n into at least k parts equals the number of
//! partitions into exactly k parts plus the number of partitions into at least k-1 parts.
//! Subtracting 1 from each part of a partition of n into k parts gives a partition of n-k
//! into k parts. These two facts together are used for this algorithm.
//!
//! More info:
//! * <https://en.wikipedia.org/wiki/Partition_(number_theory)>
//! * <https://en.wikipedia.org/wiki/Partition_function_(number_theory)>

#![allow(clippy::large_stack_arrays)]

/// Calculates the number of partitions of a positive integer using dynamic programming.
///
/// # Arguments
///
/// * `m` - A positive integer to find the number of partitions for
///
/// # Returns
///
/// The number of partitions of `m`
///
/// # Panics
///
/// Panics if `m` is not a positive integer (0 or negative)
///
/// # Examples
///
/// ```
/// # use the_algorithms_rust::dynamic_programming::partition;
/// assert_eq!(partition(5), 7);
/// assert_eq!(partition(7), 15);
/// assert_eq!(partition(100), 190569292);
/// ```
#[allow(clippy::large_stack_arrays)]
pub fn partition(m: i32) -> u128 {
    // Validate input
    assert!(m > 0, "Input must be a positive integer greater than 0");

    let m = m as usize;

    // Initialize memo table with zeros using iterative construction
    // to avoid large stack allocations
    let mut memo: Vec<Vec<u128>> = Vec::with_capacity(m + 1);
    for _ in 0..=m {
        memo.push(vec![0u128; m]);
    }

    // Base case: there's one way to partition into 0 parts (empty partition)
    for i in 0..=m {
        memo[i][0] = 1;
    }

    // Fill the memo table using dynamic programming
    for n in 0..=m {
        for k in 1..m {
            // Add partitions from k-1 (partitions with at least k-1 parts)
            memo[n][k] += memo[n][k - 1];

            // Add partitions from n-k-1 with k parts (subtract 1 from each part)
            if n > k {
                memo[n][k] += memo[n - k - 1][k];
            }
        }
    }

    memo[m][m - 1]
}

#[cfg(test)]
#[allow(clippy::large_stack_arrays)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_5() {
        assert_eq!(partition(5), 7);
    }

    #[test]
    fn test_partition_7() {
        assert_eq!(partition(7), 15);
    }

    #[test]
    #[allow(clippy::large_stack_arrays)]
    fn test_partition_100() {
        assert_eq!(partition(100), 190569292);
    }

    #[test]
    #[allow(clippy::large_stack_arrays)]
    fn test_partition_1000() {
        assert_eq!(partition(1000), 24061467864032622473692149727991);
    }

    #[test]
    #[should_panic(expected = "Input must be a positive integer greater than 0")]
    fn test_partition_negative() {
        partition(-7);
    }

    #[test]
    #[should_panic(expected = "Input must be a positive integer greater than 0")]
    fn test_partition_zero() {
        partition(0);
    }

    #[test]
    fn test_partition_small_values() {
        assert_eq!(partition(1), 1);
        assert_eq!(partition(2), 2);
        assert_eq!(partition(3), 3);
        assert_eq!(partition(4), 5);
    }
}
