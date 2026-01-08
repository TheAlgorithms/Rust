//! # Minimum Coin Change (Greedy Algorithm)
//!
//! This module implements a greedy algorithm to find the minimum number of coins
//! needed to make change for a given amount using specified denominations.
//!
//! ## Algorithm
//!
//! The greedy approach works by always selecting the largest denomination possible
//! at each step. While this approach doesn't guarantee an optimal solution for all
//! denomination systems, it works correctly for canonical coin systems (like most
//! real-world currencies including USD, EUR, INR, etc.).
//!
//! ## Time Complexity
//!
//! O(n) where n is the number of denominations
//!
//! ## Space Complexity
//!
//! O(m) where m is the number of coins in the result
//!
//! ## Example
//!
//! ```
//! # fn find_minimum_change(denominations: &[i32], value: i32) -> Vec<i32> {
//! #     if value <= 0 || denominations.is_empty() {
//! #         return Vec::new();
//! #     }
//! #     let mut remaining_value = value;
//! #     let mut result = Vec::new();
//! #     let mut sorted_denominations = denominations.to_vec();
//! #     sorted_denominations.sort_unstable_by(|a, b| b.cmp(a));
//! #     for &denomination in &sorted_denominations {
//! #         while remaining_value >= denomination {
//! #             remaining_value -= denomination;
//! #             result.push(denomination);
//! #         }
//! #     }
//! #     result
//! # }
//! let denominations = vec![1, 2, 5, 10, 20, 50, 100, 500, 2000];
//! let result = find_minimum_change(&denominations, 987);
//! assert_eq!(result, vec![500, 100, 100, 100, 100, 50, 20, 10, 5, 2]);
//! ```

/// Finds the minimum number of coins needed to make change for a given value
/// using a greedy algorithm.
///
/// # Arguments
///
/// * `denominations` - A slice of available coin denominations (must be positive integers)
/// * `value` - The target value to make change for (must be non-negative)
///
/// # Returns
///
/// A vector containing the coins used, in descending order. Returns an empty vector
/// if the value is zero or negative, or if denominations is empty.
///
/// # Examples
///
/// ```
/// # fn find_minimum_change(denominations: &[i32], value: i32) -> Vec<i32> {
/// #     if value <= 0 || denominations.is_empty() { return Vec::new(); }
/// #     let mut remaining_value = value;
/// #     let mut result = Vec::new();
/// #     let mut sorted_denominations = denominations.to_vec();
/// #     sorted_denominations.sort_unstable_by(|a, b| b.cmp(a));
/// #     for &denomination in &sorted_denominations {
/// #         while remaining_value >= denomination {
/// #             remaining_value -= denomination;
/// #             result.push(denomination);
/// #         }
/// #     }
/// #     result
/// # }
/// // Indian currency example
/// let denominations = vec![1, 2, 5, 10, 20, 50, 100, 500, 2000];
/// let result = find_minimum_change(&denominations, 987);
/// assert_eq!(result, vec![500, 100, 100, 100, 100, 50, 20, 10, 5, 2]);
/// ```
///
/// ```
/// # fn find_minimum_change(denominations: &[i32], value: i32) -> Vec<i32> {
/// #     if value <= 0 || denominations.is_empty() { return Vec::new(); }
/// #     let mut remaining_value = value;
/// #     let mut result = Vec::new();
/// #     let mut sorted_denominations = denominations.to_vec();
/// #     sorted_denominations.sort_unstable_by(|a, b| b.cmp(a));
/// #     for &denomination in &sorted_denominations {
/// #         while remaining_value >= denomination {
/// #             remaining_value -= denomination;
/// #             result.push(denomination);
/// #         }
/// #     }
/// #     result
/// # }
/// // Large amount example
/// let denominations = vec![1, 5, 10, 20, 50, 100, 200, 500, 1000, 2000];
/// let result = find_minimum_change(&denominations, 18745);
/// assert_eq!(
///     result,
///     vec![2000, 2000, 2000, 2000, 2000, 2000, 2000, 2000, 2000, 500, 200, 20, 20, 5]
/// );
/// ```
///
/// ```
/// # fn find_minimum_change(denominations: &[i32], value: i32) -> Vec<i32> {
/// #     if value <= 0 || denominations.is_empty() { return Vec::new(); }
/// #     let mut remaining_value = value;
/// #     let mut result = Vec::new();
/// #     let mut sorted_denominations = denominations.to_vec();
/// #     sorted_denominations.sort_unstable_by(|a, b| b.cmp(a));
/// #     for &denomination in &sorted_denominations {
/// #         while remaining_value >= denomination {
/// #             remaining_value -= denomination;
/// #             result.push(denomination);
/// #         }
/// #     }
/// #     result
/// # }
/// // Edge case: zero value
/// let denominations = vec![1, 2, 5, 10];
/// let result = find_minimum_change(&denominations, 0);
/// assert_eq!(result, Vec::<i32>::new());
/// ```
///
/// ```
/// # fn find_minimum_change(denominations: &[i32], value: i32) -> Vec<i32> {
/// #     if value <= 0 || denominations.is_empty() { return Vec::new(); }
/// #     let mut remaining_value = value;
/// #     let mut result = Vec::new();
/// #     let mut sorted_denominations = denominations.to_vec();
/// #     sorted_denominations.sort_unstable_by(|a, b| b.cmp(a));
/// #     for &denomination in &sorted_denominations {
/// #         while remaining_value >= denomination {
/// #             remaining_value -= denomination;
/// #             result.push(denomination);
/// #         }
/// #     }
/// #     result
/// # }
/// // Edge case: negative value
/// let denominations = vec![1, 2, 5, 10];
/// let result = find_minimum_change(&denominations, -50);
/// assert_eq!(result, Vec::<i32>::new());
/// ```
///
/// ```
/// # fn find_minimum_change(denominations: &[i32], value: i32) -> Vec<i32> {
/// #     if value <= 0 || denominations.is_empty() { return Vec::new(); }
/// #     let mut remaining_value = value;
/// #     let mut result = Vec::new();
/// #     let mut sorted_denominations = denominations.to_vec();
/// #     sorted_denominations.sort_unstable_by(|a, b| b.cmp(a));
/// #     for &denomination in &sorted_denominations {
/// #         while remaining_value >= denomination {
/// #             remaining_value -= denomination;
/// #             result.push(denomination);
/// #         }
/// #     }
/// #     result
/// # }
/// // Non-standard denominations
/// let denominations = vec![1, 5, 100, 500, 1000];
/// let result = find_minimum_change(&denominations, 456);
/// assert_eq!(
///     result,
///     vec![100, 100, 100, 100, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1]
/// );
/// ```
pub fn find_minimum_change(denominations: &[i32], value: i32) -> Vec<i32> {
    // Handle edge cases
    if value <= 0 || denominations.is_empty() {
        return Vec::new();
    }

    let mut remaining_value = value;
    let mut result = Vec::new();

    // Sort denominations in descending order for greedy selection
    let mut sorted_denominations = denominations.to_vec();
    sorted_denominations.sort_unstable_by(|a, b| b.cmp(a));

    // Greedily select the largest denomination at each step
    for &denomination in &sorted_denominations {
        while remaining_value >= denomination {
            remaining_value -= denomination;
            result.push(denomination);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indian_currency_standard() {
        let denominations = vec![1, 2, 5, 10, 20, 50, 100, 500, 2000];
        let result = find_minimum_change(&denominations, 987);
        assert_eq!(result, vec![500, 100, 100, 100, 100, 50, 20, 10, 5, 2]);
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_large_amount() {
        let denominations = vec![1, 5, 10, 20, 50, 100, 200, 500, 1000, 2000];
        let result = find_minimum_change(&denominations, 18745);
        assert_eq!(
            result,
            vec![2000, 2000, 2000, 2000, 2000, 2000, 2000, 2000, 2000, 500, 200, 20, 20, 5]
        );
        assert_eq!(result.iter().sum::<i32>(), 18745);
    }

    #[test]
    fn test_zero_value() {
        let denominations = vec![1, 2, 5, 10, 20, 50, 100, 500, 2000];
        let result = find_minimum_change(&denominations, 0);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_negative_value() {
        let denominations = vec![1, 2, 5, 10, 20, 50, 100, 500, 2000];
        let result = find_minimum_change(&denominations, -98);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_non_standard_denominations() {
        let denominations = vec![1, 5, 100, 500, 1000];
        let result = find_minimum_change(&denominations, 456);
        assert_eq!(
            result,
            vec![100, 100, 100, 100, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1]
        );
        assert_eq!(result.iter().sum::<i32>(), 456);
    }

    #[test]
    fn test_single_denomination() {
        let denominations = vec![5];
        let result = find_minimum_change(&denominations, 25);
        assert_eq!(result, vec![5, 5, 5, 5, 5]);
    }

    #[test]
    fn test_exact_denomination() {
        let denominations = vec![1, 5, 10, 25, 50, 100];
        let result = find_minimum_change(&denominations, 100);
        assert_eq!(result, vec![100]);
    }

    #[test]
    fn test_empty_denominations() {
        let denominations: Vec<i32> = vec![];
        let result = find_minimum_change(&denominations, 100);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_unsorted_denominations() {
        let denominations = vec![100, 1, 50, 5, 20, 10, 2];
        let result = find_minimum_change(&denominations, 178);
        assert_eq!(result, vec![100, 50, 20, 5, 2, 1]);
        assert_eq!(result.iter().sum::<i32>(), 178);
    }

    #[test]
    fn test_usd_currency() {
        let denominations = vec![1, 5, 10, 25, 50, 100]; // cents
        let result = find_minimum_change(&denominations, 99);
        assert_eq!(result, vec![50, 25, 10, 10, 1, 1, 1, 1]);
        assert_eq!(result.len(), 8);
    }
}
