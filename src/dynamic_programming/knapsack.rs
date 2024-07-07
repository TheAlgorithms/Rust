//! This module provides functionality to solve the knapsack problem using dynamic programming.
//! It includes structures for items and solutions, and functions to compute the optimal solution.

use std::cmp::Ordering;

/// Represents an item with a weight and a value.
#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    weight: usize,
    value: usize,
}

/// Represents the solution to the knapsack problem.
#[derive(Debug, PartialEq, Eq)]
pub struct KnapsackSolution {
    /// The optimal profit obtained.
    optimal_profit: usize,
    /// The total weight of items included in the solution.
    total_weight: usize,
    /// The indices of items included in the solution. Indices might not be unique.
    item_indices: Vec<usize>,
}

/// Solves the knapsack problem and returns the optimal profit, total weight, and indices of items included.
///
/// # Arguments:
/// * `capacity` - The maximum weight capacity of the knapsack.
/// * `items` - A vector of `Item` structs, each representing an item with weight and value.
///
/// # Returns:
/// A `KnapsackSolution` struct containing:
/// - `optimal_profit` - The maximum profit achievable with the given capacity and items.
/// - `total_weight` - The total weight of items included in the solution.
/// - `item_indices` - Indices of items included in the solution. Indices might not be unique.
///
/// # Note:
/// The indices of items in the solution might not be unique.
/// This function assumes that `items` is non-empty.
///
/// # Complexity:
/// - Time complexity: O(num_items * capacity)
/// - Space complexity: O(num_items * capacity)
///
/// where `num_items` is the number of items and `capacity` is the knapsack capacity.
pub fn knapsack(capacity: usize, items: Vec<Item>) -> KnapsackSolution {
    let num_items = items.len();
    let item_weights: Vec<usize> = items.iter().map(|item| item.weight).collect();
    let item_values: Vec<usize> = items.iter().map(|item| item.value).collect();

    let knapsack_matrix = generate_knapsack_matrix(capacity, &item_weights, &item_values);
    let items_included =
        retrieve_knapsack_items(&item_weights, &knapsack_matrix, num_items, capacity);

    let total_weight = items_included
        .iter()
        .map(|&index| item_weights[index - 1])
        .sum();

    KnapsackSolution {
        optimal_profit: knapsack_matrix[num_items][capacity],
        total_weight,
        item_indices: items_included,
    }
}

/// Generates the knapsack matrix (`num_items`, `capacity`) with maximum values.
///
/// # Arguments:
///   * `capacity` - knapsack capacity
///   * `item_weights` - weights of each item
///   * `item_values` - values of each item
fn generate_knapsack_matrix(
    capacity: usize,
    item_weights: &[usize],
    item_values: &[usize],
) -> Vec<Vec<usize>> {
    let num_items = item_weights.len();

    (0..=num_items).fold(
        vec![vec![0; capacity + 1]; num_items + 1],
        |mut matrix, item_index| {
            (0..=capacity).for_each(|current_capacity| {
                matrix[item_index][current_capacity] = if item_index == 0 || current_capacity == 0 {
                    0
                } else if item_weights[item_index - 1] <= current_capacity {
                    usize::max(
                        item_values[item_index - 1]
                            + matrix[item_index - 1]
                                [current_capacity - item_weights[item_index - 1]],
                        matrix[item_index - 1][current_capacity],
                    )
                } else {
                    matrix[item_index - 1][current_capacity]
                };
            });
            matrix
        },
    )
}

/// Retrieves the indices of items included in the optimal knapsack solution.
///
/// # Arguments:
///   * `item_weights` - weights of each item
///   * `knapsack_matrix` - knapsack matrix with maximum values
///   * `item_index` - number of items to consider (initially the total number of items)
///   * `remaining_capacity` - remaining capacity of the knapsack
///
/// # Returns
/// A vector of item indices included in the optimal solution. The indices might not be unique.
fn retrieve_knapsack_items(
    item_weights: &[usize],
    knapsack_matrix: &[Vec<usize>],
    item_index: usize,
    remaining_capacity: usize,
) -> Vec<usize> {
    match item_index {
        0 => vec![],
        _ => {
            let current_value = knapsack_matrix[item_index][remaining_capacity];
            let previous_value = knapsack_matrix[item_index - 1][remaining_capacity];

            match current_value.cmp(&previous_value) {
                Ordering::Greater => {
                    let mut knap = retrieve_knapsack_items(
                        item_weights,
                        knapsack_matrix,
                        item_index - 1,
                        remaining_capacity - item_weights[item_index - 1],
                    );
                    knap.push(item_index);
                    knap
                }
                Ordering::Equal | Ordering::Less => retrieve_knapsack_items(
                    item_weights,
                    knapsack_matrix,
                    item_index - 1,
                    remaining_capacity,
                ),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! knapsack_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (capacity, items, expected) = $test_case;
                    assert_eq!(expected, knapsack(capacity, items));
                }
            )*
        }
    }

    knapsack_tests! {
        test_basic_knapsack_small: (
            165,
            vec![
                Item { weight: 23, value: 92 },
                Item { weight: 31, value: 57 },
                Item { weight: 29, value: 49 },
                Item { weight: 44, value: 68 },
                Item { weight: 53, value: 60 },
                Item { weight: 38, value: 43 },
                Item { weight: 63, value: 67 },
                Item { weight: 85, value: 84 },
                Item { weight: 89, value: 87 },
                Item { weight: 82, value: 72 }
            ],
            KnapsackSolution {
                optimal_profit: 309,
                total_weight: 165,
                item_indices: vec![1, 2, 3, 4, 6]
            }
        ),
        test_basic_knapsack_tiny: (
            26,
            vec![
                Item { weight: 12, value: 24 },
                Item { weight: 7, value: 13 },
                Item { weight: 11, value: 23 },
                Item { weight: 8, value: 15 },
                Item { weight: 9, value: 16 }
            ],
            KnapsackSolution {
                optimal_profit: 51,
                total_weight: 26,
                item_indices: vec![2, 3, 4]
            }
        ),
        test_basic_knapsack_medium: (
            190,
            vec![
                Item { weight: 56, value: 50 },
                Item { weight: 59, value: 50 },
                Item { weight: 80, value: 64 },
                Item { weight: 64, value: 46 },
                Item { weight: 75, value: 50 },
                Item { weight: 17, value: 5 }
            ],
            KnapsackSolution {
                optimal_profit: 150,
                total_weight: 190,
                item_indices: vec![1, 2, 5]
            }
        ),
        test_diverse_weights_values_small: (
            50,
            vec![
                Item { weight: 31, value: 70 },
                Item { weight: 10, value: 20 },
                Item { weight: 20, value: 39 },
                Item { weight: 19, value: 37 },
                Item { weight: 4, value: 7 },
                Item { weight: 3, value: 5 },
                Item { weight: 6, value: 10 }
            ],
            KnapsackSolution {
                optimal_profit: 107,
                total_weight: 50,
                item_indices: vec![1, 4]
            }
        ),
        test_diverse_weights_values_medium: (
            104,
            vec![
                Item { weight: 25, value: 350 },
                Item { weight: 35, value: 400 },
                Item { weight: 45, value: 450 },
                Item { weight: 5, value: 20 },
                Item { weight: 25, value: 70 },
                Item { weight: 3, value: 8 },
                Item { weight: 2, value: 5 },
                Item { weight: 2, value: 5 }
            ],
            KnapsackSolution {
                optimal_profit: 900,
                total_weight: 104,
                item_indices: vec![1, 3, 4, 5, 7, 8]
            }
        ),
        test_high_value_items: (
            170,
            vec![
                Item { weight: 41, value: 442 },
                Item { weight: 50, value: 525 },
                Item { weight: 49, value: 511 },
                Item { weight: 59, value: 593 },
                Item { weight: 55, value: 546 },
                Item { weight: 57, value: 564 },
                Item { weight: 60, value: 617 }
            ],
            KnapsackSolution {
                optimal_profit: 1735,
                total_weight: 169,
                item_indices: vec![2, 4, 7]
            }
        ),
        test_large_knapsack: (
            750,
            vec![
                Item { weight: 70, value: 135 },
                Item { weight: 73, value: 139 },
                Item { weight: 77, value: 149 },
                Item { weight: 80, value: 150 },
                Item { weight: 82, value: 156 },
                Item { weight: 87, value: 163 },
                Item { weight: 90, value: 173 },
                Item { weight: 94, value: 184 },
                Item { weight: 98, value: 192 },
                Item { weight: 106, value: 201 },
                Item { weight: 110, value: 210 },
                Item { weight: 113, value: 214 },
                Item { weight: 115, value: 221 },
                Item { weight: 118, value: 229 },
                Item { weight: 120, value: 240 }
            ],
            KnapsackSolution {
                optimal_profit: 1458,
                total_weight: 749,
                item_indices: vec![1, 3, 5, 7, 8, 9, 14, 15]
            }
        ),
        test_zero_capacity: (
            0,
            vec![
                Item { weight: 1, value: 1 },
                Item { weight: 2, value: 2 },
                Item { weight: 3, value: 3 }
            ],
            KnapsackSolution {
                optimal_profit: 0,
                total_weight: 0,
                item_indices: vec![]
            }
        ),
        test_very_small_capacity: (
            1,
            vec![
                Item { weight: 10, value: 1 },
                Item { weight: 20, value: 2 },
                Item { weight: 30, value: 3 }
            ],
            KnapsackSolution {
                optimal_profit: 0,
                total_weight: 0,
                item_indices: vec![]
            }
        ),
        test_no_items: (
            1,
            vec![],
            KnapsackSolution {
                optimal_profit: 0,
                total_weight: 0,
                item_indices: vec![]
            }
        ),
        test_item_too_heavy: (
            1,
            vec![
                Item { weight: 2, value: 100 }
            ],
            KnapsackSolution {
                optimal_profit: 0,
                total_weight: 0,
                item_indices: vec![]
            }
        ),
        test_greedy_algorithm_does_not_work: (
            10,
            vec![
                Item { weight: 10, value: 15 },
                Item { weight: 6, value: 7 },
                Item { weight: 4, value: 9 }
            ],
            KnapsackSolution {
                optimal_profit: 16,
                total_weight: 10,
                item_indices: vec![2, 3]
            }
        ),
        test_greedy_algorithm_does_not_work_weight_smaller_than_capacity: (
            10,
            vec![
                Item { weight: 10, value: 15 },
                Item { weight: 1, value: 9 },
                Item { weight: 2, value: 7 }
            ],
            KnapsackSolution {
                optimal_profit: 16,
                total_weight: 3,
                item_indices: vec![2, 3]
            }
        ),
    }
}
