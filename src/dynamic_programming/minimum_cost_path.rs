use std::cmp::min;

/// Computes the minimum cost path from the top-left to the bottom-right
/// corner of a matrix, where movement is restricted to right and down directions.
///
/// The function calculates the minimum path cost by updating a `cost` vector that
/// stores cumulative path costs for the current row as it iterates through each
/// row of the input `matrix`.
///
/// # Arguments
///
/// * `matrix` - A 2D vector of positive integers, where each element represents
///              the cost to step on that cell.
///
/// # Returns
///
/// * A single `usize` value representing the minimum path cost to reach the
///   bottom-right corner from the top-left corner of the matrix.
///
/// # Complexity
///
/// * Time complexity: `O(m * n)`, where `m` is the number of rows
///   and `n` is the number of columns in the input matrix.
/// * Space complexity: `O(n)`, as only a single row of cumulative costs
///   is stored at any time.
pub fn minimum_cost_path(matrix: Vec<Vec<usize>>) -> usize {
    // Initialize the first row of the cost vector using a scan over the first row of matrix
    let mut cost = matrix[0]
        .iter()
        .scan(0, |acc, &val| {
            *acc += val;
            Some(*acc)
        })
        .collect::<Vec<_>>();

    // Process each row from the second to the last
    for row in matrix.iter().skip(1) {
        // Update the first element of cost for this row
        cost[0] += row[0];

        // Update the rest of the elements in the current row of cost
        for col in 1..matrix[0].len() {
            cost[col] = row[col] + min(cost[col - 1], cost[col]);
        }
    }

    // The last element in cost contains the minimum path cost to the bottom-right corner
    cost[matrix[0].len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! minimum_cost_path_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (matrix, expected) = $test_case;
                    assert_eq!(minimum_cost_path(matrix), expected);
                }
            )*
        };
    }

    minimum_cost_path_tests! {
        basic: (
            vec![
                vec![2, 1, 4],
                vec![2, 1, 3],
                vec![3, 2, 1]
            ],
            7
        ),
        single_element: (
            vec![
                vec![5]
            ],
            5
        ),
        single_row: (
            vec![
                vec![1, 3, 2, 1, 5]
            ],
            12
        ),
        single_column: (
            vec![
                vec![1],
                vec![3],
                vec![2],
                vec![1],
                vec![5]],
            12
        ),
        large_matrix: (
            vec![
                vec![1, 3, 1, 5],
                vec![2, 1, 4, 2],
                vec![3, 2, 1, 3],
                vec![4, 3, 2, 1]
            ],
            10
        ),
        uniform_matrix: (
            vec![
                vec![1, 1, 1],
                vec![1, 1, 1],
                vec![1, 1, 1]
            ],
            5
        ),
        increasing_values: (
            vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9]
            ],
            21
        ),
        high_cost_path: (
            vec![
                vec![1, 100, 1],
                vec![1, 100, 1],
                vec![1, 1, 1]
            ],
            5
        ),
        complex_matrix: (
            vec![
                vec![5, 9, 6, 8],
                vec![1, 4, 7, 3],
                vec![2, 1, 8, 2],
                vec![3, 6, 9, 4]
            ],
            23
        ),
    }
}
