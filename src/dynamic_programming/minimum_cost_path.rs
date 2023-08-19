/// Minimum Cost Path via Dynamic Programming

/// Find the minimum cost traced by all possible paths from top left to bottom right in
/// a given matrix, by allowing only right and down movement

/// For example, in matrix,
/// [2, 1, 4]
/// [2, 1, 3]
/// [3, 2, 1]
/// The minimum cost path is 7

/// # Arguments:
///   * `matrix` - The input matrix.
/// # Complexity
///   - time complexity: O( rows * columns ),
///   - space complexity: O( rows * columns )
use std::cmp::min;

pub fn minimum_cost_path(mut matrix: Vec<Vec<usize>>) -> usize {
    // Add rows and columns variables for better readability
    let rows = matrix.len();
    let columns = matrix[0].len();

    // Preprocessing the first row
    for i in 1..columns {
        matrix[0][i] += matrix[0][i - 1];
    }

    // Preprocessing the first column
    for i in 1..rows {
        matrix[i][0] += matrix[i - 1][0];
    }

    // Updating path cost for the remaining positions
    // For each position, cost to reach it from top left is
    // Sum of value of that position and minimum of upper and left position value

    for i in 1..rows {
        for j in 1..columns {
            matrix[i][j] += min(matrix[i - 1][j], matrix[i][j - 1]);
        }
    }

    // Return cost for bottom right element
    matrix[rows - 1][columns - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // For test case in example
        let matrix = vec![vec![2, 1, 4], vec![2, 1, 3], vec![3, 2, 1]];
        assert_eq!(minimum_cost_path(matrix), 7);

        // For a randomly generated matrix
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(minimum_cost_path(matrix), 12);
    }

    #[test]
    fn one_element_matrix() {
        let matrix = vec![vec![2]];
        assert_eq!(minimum_cost_path(matrix), 2);
    }

    #[test]
    fn one_row() {
        let matrix = vec![vec![1, 3, 2, 1, 5]];
        assert_eq!(minimum_cost_path(matrix), 12);
    }

    #[test]
    fn one_column() {
        let matrix = vec![vec![1], vec![3], vec![2], vec![1], vec![5]];
        assert_eq!(minimum_cost_path(matrix), 12);
    }
}
