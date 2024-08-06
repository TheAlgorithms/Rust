//! This module provides functions for solving the rod-cutting problem using dynamic programming.
use std::cmp::max;

/// Calculates the maximum possible profit from cutting a rod into pieces of varying lengths.
///
/// Returns the maximum profit achievable by cutting a rod into pieces such that the profit from each
/// piece is determined by its length and predefined prices.
///
/// # Complexity
/// - Time complexity: `O(n^2)`
/// - Space complexity: `O(n)`
///
/// where `n` is the number of different rod lengths considered.
pub fn rod_cut(prices: &[usize]) -> usize {
    if prices.is_empty() {
        return 0;
    }

    (1..=prices.len()).fold(vec![0; prices.len() + 1], |mut max_profit, rod_length| {
        max_profit[rod_length] = (1..=rod_length)
            .map(|cut_position| prices[cut_position - 1] + max_profit[rod_length - cut_position])
            .fold(prices[rod_length - 1], |max_price, current_price| {
                max(max_price, current_price)
            });
        max_profit
    })[prices.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! rod_cut_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected_output) = $test_case;
                    assert_eq!(expected_output, rod_cut(input));
                }
            )*
        };
    }

    rod_cut_tests! {
        test_empty_prices: (&[], 0),
        test_example_with_three_prices: (&[5, 8, 2], 15),
        test_example_with_four_prices: (&[1, 5, 8, 9], 10),
        test_example_with_five_prices: (&[5, 8, 2, 1, 7], 25),
        test_all_zeros_except_last: (&[0, 0, 0, 0, 0, 87], 87),
        test_descending_prices: (&[7, 6, 5, 4, 3, 2, 1], 49),
        test_varied_prices: (&[1, 5, 8, 9, 10, 17, 17, 20], 22),
        test_complex_prices: (&[6, 4, 8, 2, 5, 8, 2, 3, 7, 11], 60),
        test_increasing_prices: (&[1, 5, 8, 9, 10, 17, 17, 20, 24, 30], 30),
        test_large_range_prices: (&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], 12),
        test_single_length_price: (&[5], 5),
        test_zero_length_price: (&[0], 0),
        test_repeated_prices: (&[5, 5, 5, 5], 20),
        test_no_profit: (&[0, 0, 0, 0], 0),
        test_large_input: (&[1; 1000], 1000),
        test_all_zero_input: (&[0; 100], 0),
        test_very_large_prices: (&[1000000, 2000000, 3000000], 3000000),
        test_greedy_does_not_work: (&[2, 5, 7, 8], 10),
    }
}
