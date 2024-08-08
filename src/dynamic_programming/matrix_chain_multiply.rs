//! This module implements a dynamic programming solution to find the minimum
//! number of multiplications needed to multiply a chain of matrices with given dimensions.
//!
//! The algorithm uses a dynamic programming approach with tabulation to calculate the minimum
//! number of multiplications required for matrix chain multiplication.
//!
//! # Time Complexity
//!
//! The algorithm runs in O(n^3) time complexity and O(n^2) space complexity, where n is the
//! number of matrices.

/// Custom error types for matrix chain multiplication
#[derive(Debug, PartialEq)]
pub enum MatrixChainMultiplicationError {
    EmptyDimensions,
    InsufficientDimensions,
}

/// Calculates the minimum number of scalar multiplications required to multiply a chain
/// of matrices with given dimensions.
///
/// # Arguments
///
/// * `dimensions`: A vector where each element represents the dimensions of consecutive matrices
///   in the chain. For example, [1, 2, 3, 4] represents matrices of dimensions (1x2), (2x3), and (3x4).
///
/// # Returns
///
/// The minimum number of scalar multiplications needed to compute the product of the matrices
/// in the optimal order.
///
/// # Errors
///
/// Returns an error if the input is invalid (i.e., empty or length less than 2).
pub fn matrix_chain_multiply(
    dimensions: Vec<usize>,
) -> Result<usize, MatrixChainMultiplicationError> {
    if dimensions.is_empty() {
        return Err(MatrixChainMultiplicationError::EmptyDimensions);
    }

    if dimensions.len() == 1 {
        return Err(MatrixChainMultiplicationError::InsufficientDimensions);
    }

    let mut min_operations = vec![vec![0; dimensions.len()]; dimensions.len()];

    (2..dimensions.len()).for_each(|chain_len| {
        (0..dimensions.len() - chain_len).for_each(|start| {
            let end = start + chain_len;
            min_operations[start][end] = (start + 1..end)
                .map(|split| {
                    min_operations[start][split]
                        + min_operations[split][end]
                        + dimensions[start] * dimensions[split] * dimensions[end]
                })
                .min()
                .unwrap_or(usize::MAX);
        });
    });

    Ok(min_operations[0][dimensions.len() - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_cases {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $test_case;
                    assert_eq!(matrix_chain_multiply(input.clone()), expected);
                    assert_eq!(matrix_chain_multiply(input.into_iter().rev().collect()), expected);
                }
            )*
        };
    }

    test_cases! {
        basic_chain_of_matrices: (vec![1, 2, 3, 4], Ok(18)),
        chain_of_large_matrices: (vec![40, 20, 30, 10, 30], Ok(26000)),
        long_chain_of_matrices: (vec![1, 2, 3, 4, 3, 5, 7, 6, 10], Ok(182)),
        complex_chain_of_matrices: (vec![4, 10, 3, 12, 20, 7], Ok(1344)),
        empty_dimensions_input: (vec![], Err(MatrixChainMultiplicationError::EmptyDimensions)),
        single_dimensions_input: (vec![10], Err(MatrixChainMultiplicationError::InsufficientDimensions)),
        single_matrix_input: (vec![10, 20], Ok(0)),
    }
}
