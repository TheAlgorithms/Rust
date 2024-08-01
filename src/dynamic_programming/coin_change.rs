//! This module provides a solution to the coin change problem using dynamic programming.
//! The `coin_change` function calculates the fewest number of coins required to make up
//! a given amount using a specified set of coin denominations.
//!
//! The implementation leverages dynamic programming to build up solutions for smaller
//! amounts and combines them to solve for larger amounts. It ensures optimal substructure
//! and overlapping subproblems are efficiently utilized to achieve the solution.

//! # Complexity
//! - Time complexity: O(amount * coins.length)
//! - Space complexity: O(amount)

/// Returns the fewest number of coins needed to make up the given amount using the provided coin denominations.
/// If the amount cannot be made up by any combination of the coins, returns `None`.
///
/// # Arguments
/// * `coins` - A slice of coin denominations.
/// * `amount` - The total amount of money to be made up.
///
/// # Returns
/// * `Option<usize>` - The minimum number of coins required to make up the amount, or `None` if it's not possible.
///
/// # Complexity
/// * Time complexity: O(amount * coins.length)
/// * Space complexity: O(amount)
pub fn coin_change(coins: &[usize], amount: usize) -> Option<usize> {
    let mut min_coins = vec![None; amount + 1];
    min_coins[0] = Some(0);

    (0..=amount).for_each(|curr_amount| {
        coins
            .iter()
            .filter(|&&coin| curr_amount >= coin)
            .for_each(|&coin| {
                if let Some(prev_min_coins) = min_coins[curr_amount - coin] {
                    min_coins[curr_amount] = Some(
                        min_coins[curr_amount].map_or(prev_min_coins + 1, |curr_min_coins| {
                            curr_min_coins.min(prev_min_coins + 1)
                        }),
                    );
                }
            });
    });

    min_coins[amount]
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! coin_change_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (coins, amount, expected) = $test_case;
                    assert_eq!(expected, coin_change(&coins, amount));
                }
            )*
        }
    }

    coin_change_tests! {
        test_basic_case: (vec![1, 2, 5], 11, Some(3)),
        test_multiple_denominations: (vec![2, 3, 5, 7, 11], 119, Some(12)),
        test_empty_coins: (vec![], 1, None),
        test_zero_amount: (vec![1, 2, 3], 0, Some(0)),
        test_no_solution_small_coin: (vec![2], 3, None),
        test_no_solution_large_coin: (vec![10, 20, 50, 100], 5, None),
        test_single_coin_large_amount: (vec![1], 100, Some(100)),
        test_large_amount_multiple_coins: (vec![1, 2, 5], 10000, Some(2000)),
        test_no_combination_possible: (vec![3, 7], 5, None),
        test_exact_combination: (vec![1, 3, 4], 6, Some(2)),
        test_large_denomination_multiple_coins: (vec![10, 50, 100], 1000, Some(10)),
        test_small_amount_not_possible: (vec![5, 10], 1, None),
        test_non_divisible_amount: (vec![2], 3, None),
        test_all_multiples: (vec![1, 2, 4, 8], 15, Some(4)),
        test_large_amount_mixed_coins: (vec![1, 5, 10, 25], 999, Some(45)),
        test_prime_coins_and_amount: (vec![2, 3, 5, 7], 17, Some(3)),
        test_coins_larger_than_amount: (vec![5, 10, 20], 1, None),
        test_repeating_denominations: (vec![1, 1, 1, 5], 8, Some(4)),
        test_non_standard_denominations: (vec![1, 4, 6, 9], 15, Some(2)),
        test_very_large_denominations: (vec![1000, 2000, 5000], 1, None),
        test_large_amount_performance: (vec![1, 5, 10, 25, 50, 100, 200, 500], 9999, Some(29)),
        test_powers_of_two: (vec![1, 2, 4, 8, 16, 32, 64], 127, Some(7)),
        test_fibonacci_sequence: (vec![1, 2, 3, 5, 8, 13, 21, 34], 55, Some(2)),
        test_mixed_small_large: (vec![1, 100, 1000, 10000], 11001, Some(3)),
        test_impossible_combinations: (vec![2, 4, 6, 8], 7, None),
        test_greedy_approach_does_not_work: (vec![1, 12, 20], 24, Some(2)),
        test_zero_denominations_no_solution: (vec![0], 1, None),
        test_zero_denominations_solution: (vec![0], 0, Some(0)),
    }
}
