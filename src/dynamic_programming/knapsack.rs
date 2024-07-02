//! Solves the knapsack problem
use std::cmp::Ordering;

/// Solves the knapsack problem and returns the optimal profit, total weight, and indices of items included.
///
/// # Arguments:
///   * `capacity` - knapsack capacity
///   * `item_weights` - weights of each item
///   * `item_values` - values of each item
///
/// # Returns
///   A tuple containing:
///   - optimal profit
///   - total weight of the optimal knapsack
///   - indices of items included (from 1 to `num_items`)
///
/// # Complexity
///   - Time complexity: O(num_items * capacity)
///   - Space complexity: O(num_items * capacity)
///
/// where `num_items` is the number of items and `capacity` is the knapsack capacity.
pub fn knapsack(
    capacity: usize,
    item_weights: Vec<usize>,
    item_values: Vec<usize>,
) -> (usize, usize, Vec<usize>) {
    assert_eq!(item_weights.len(), item_values.len(), "Number of items in the list of weights doesn't match the number of items in the list of values!");

    let num_items = item_weights.len();
    let knapsack_matrix = generate_knapsack_matrix(capacity, &item_weights, &item_values);
    let items = retrieve_knapsack_items(&item_weights, &knapsack_matrix, num_items, capacity);

    let total_weight = items.iter().map(|&index| item_weights[index - 1]).sum();

    (knapsack_matrix[num_items][capacity], total_weight, items)
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
                    let (capacity, weights, values, expected) = $test_case;
                    assert_eq!(expected, knapsack(capacity, weights, values));
                }
            )*
        }
    }

    knapsack_tests! {
        test_basic_knapsack_small: (
            165,
            vec![23, 31, 29, 44, 53, 38, 63, 85, 89, 82],
            vec![92, 57, 49, 68, 60, 43, 67, 84, 87, 72],
            (309, 165, vec![1, 2, 3, 4, 6])
        ),
        test_basic_knapsack_tiny: (
            26,
            vec![12, 7, 11, 8, 9],
            vec![24, 13, 23, 15, 16],
            (51, 26, vec![2, 3, 4])
        ),
        test_basic_knapsack_medium: (
            190,
            vec![56, 59, 80, 64, 75, 17],
            vec![50, 50, 64, 46, 50, 5],
            (150, 190, vec![1, 2, 5])
        ),
        test_diverse_weights_values_small: (
            50,
            vec![31, 10, 20, 19, 4, 3, 6],
            vec![70, 20, 39, 37, 7, 5, 10],
            (107, 50, vec![1, 4])
        ),
        test_diverse_weights_values_medium: (
            104,
            vec![25, 35, 45, 5, 25, 3, 2, 2],
            vec![350, 400, 450, 20, 70, 8, 5, 5],
            (900, 104, vec![1, 3, 4, 5, 7, 8])
        ),
        test_high_value_items: (
            170,
            vec![41, 50, 49, 59, 55, 57, 60],
            vec![442, 525, 511, 593, 546, 564, 617],
            (1735, 169, vec![2, 4, 7])
        ),
        test_large_knapsack: (
            750,
            vec![
                70, 73, 77, 80, 82, 87, 90, 94, 98, 106, 110, 113, 115, 118, 120
            ],
            vec![
                135, 139, 149, 150, 156, 163, 173, 184, 192, 201, 210, 214, 221, 229, 240
            ],
            (1458, 749, vec![1, 3, 5, 7, 8, 9, 14, 15])
        ),
    }
}
