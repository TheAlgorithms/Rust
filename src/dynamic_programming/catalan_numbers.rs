//! Catalan Numbers using Dynamic Programming
//!
//! The Catalan numbers are a sequence of positive integers that appear in many
//! counting problems in combinatorics. Such problems include counting:
//! - The number of Dyck words of length 2n
//! - The number of well-formed expressions with n pairs of parentheses
//!   (e.g., `()()` is valid but `())(` is not)
//! - The number of different ways n + 1 factors can be completely parenthesized
//!   (e.g., for n = 2, C(n) = 2 and (ab)c and a(bc) are the two valid ways)
//! - The number of full binary trees with n + 1 leaves
//!
//! A Catalan number satisfies the following recurrence relation:
//! - C(0) = C(1) = 1
//! - C(n) = sum(C(i) * C(n-i-1)), from i = 0 to n-1
//!
//! Sources:
//! - [Brilliant.org](https://brilliant.org/wiki/catalan-numbers/)
//! - [Wikipedia](https://en.wikipedia.org/wiki/Catalan_number)

/// Computes the Catalan number sequence from 0 through `upper_limit`.
///
/// # Arguments
///
/// * `upper_limit` - The upper limit for the Catalan sequence (must be ≥ 0)
///
/// # Returns
///
/// A vector containing Catalan numbers from C(0) to C(upper_limit)
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::dynamic_programming::catalan_numbers;
///
/// assert_eq!(catalan_numbers(5), vec![1, 1, 2, 5, 14, 42]);
/// assert_eq!(catalan_numbers(2), vec![1, 1, 2]);
/// assert_eq!(catalan_numbers(0), vec![1]);
/// ```
///
/// # Panics
///
/// Panics if `upper_limit` would cause integer overflow during computation.
pub fn catalan_numbers(upper_limit: usize) -> Vec<u64> {
    let mut catalan_list = vec![0u64; upper_limit + 1];

    // Base case: C(0) = 1
    catalan_list[0] = 1;

    // Base case: C(1) = 1
    if upper_limit > 0 {
        catalan_list[1] = 1;
    }

    // Recurrence relation: C(i) = sum(C(j) * C(i-j-1)), from j = 0 to i-1
    for i in 2..=upper_limit {
        for j in 0..i {
            catalan_list[i] += catalan_list[j] * catalan_list[i - j - 1];
        }
    }

    catalan_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalan_numbers_basic() {
        assert_eq!(catalan_numbers(5), vec![1, 1, 2, 5, 14, 42]);
        assert_eq!(catalan_numbers(2), vec![1, 1, 2]);
        assert_eq!(catalan_numbers(0), vec![1]);
    }

    #[test]
    fn test_catalan_numbers_single() {
        assert_eq!(catalan_numbers(1), vec![1, 1]);
    }

    #[test]
    fn test_catalan_numbers_extended() {
        let result = catalan_numbers(10);
        assert_eq!(result.len(), 11);
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 1);
        assert_eq!(result[2], 2);
        assert_eq!(result[3], 5);
        assert_eq!(result[4], 14);
        assert_eq!(result[5], 42);
        assert_eq!(result[6], 132);
        assert_eq!(result[7], 429);
        assert_eq!(result[8], 1430);
        assert_eq!(result[9], 4862);
        assert_eq!(result[10], 16796);
    }

    #[test]
    fn test_catalan_first_few() {
        // Verify the first few Catalan numbers match known values
        assert_eq!(catalan_numbers(3), vec![1, 1, 2, 5]);
        assert_eq!(catalan_numbers(4), vec![1, 1, 2, 5, 14]);
    }
}
