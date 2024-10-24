use std::cmp::Ordering;

/// Custom error type to represent errors related to matrix validation.
#[derive(Debug, PartialEq, Eq)]
pub enum MatrixError {
    NonRectangularInput,
    NotSorted,
}

/// Checks if the given matrix (vector of vectors) is sorted row-wise and column-wise.
///
/// A matrix is considered sorted if
///
/// * Each row is sorted in non-decreasing order.
/// * Each column is sorted in non-decreasing order.
///
/// # Arguments
///
/// * `matrix` - A vector of vectors representing the matrix to check.
///
/// # Returns
///
/// Returns `true` if the matrix is sorted both row-wise and column-wise. Otherwise, returns `false`.
fn is_sorted(matrix: &[Vec<isize>]) -> bool {
    if matrix.is_empty() || matrix.iter().all(|row| row.is_empty()) {
        return true;
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    // Check if rows are sorted.
    for row in matrix {
        if row.windows(2).any(|w| w[0] > w[1]) {
            return false;
        }
    }

    // Check if columns are sorted.
    for col in 0..cols {
        for row in 1..rows {
            if matrix[row - 1][col] > matrix[row][col] {
                return false;
            }
        }
    }

    true
}

/// Performs Saddleback search on a sorted matrix represented as a vector of vectors.
///
/// The Saddleback search algorithm finds the position of a target element in a matrix where
/// each row and each column is sorted in ascending order. The search starts from the top-right
/// corner of the matrix and moves left or down based on comparisons with the target element.
///
/// Optionally, the matrix can be validated for being sorted before the search starts.
///
/// # Arguments
///
/// * `matrix` - A vector of vectors representing the sorted matrix.
/// * `element` - The target element to search for.
/// * `check_sorted` - If true, verifies that the matrix is sorted before performing the search.
///
/// # Returns
///
/// Returns `Ok(Some((row, column)))` where both indices are 0-based if the element is found.
/// Returns `Ok(None)` if the element is not found.
/// Returns `Err(MatrixError)` if the matrix is not rectangular or not sorted.
pub fn saddleback_search(
    matrix: &[Vec<isize>],
    element: isize,
    check_sorted: bool,
) -> Result<Option<(usize, usize)>, MatrixError> {
    if matrix.is_empty() || matrix.iter().all(|row| row.is_empty()) {
        return Ok(None);
    }

    if matrix.iter().any(|row| row.len() != matrix[0].len()) {
        return Err(MatrixError::NonRectangularInput);
    }

    if check_sorted && !is_sorted(matrix) {
        return Err(MatrixError::NotSorted);
    }

    let mut left_index = 0;
    let mut right_index = matrix[0].len() - 1;

    while left_index < matrix.len() {
        match element.cmp(&matrix[left_index][right_index]) {
            Ordering::Equal => return Ok(Some((left_index, right_index))),
            Ordering::Greater => {
                left_index += 1;
            }
            Ordering::Less => {
                if right_index == 0 {
                    break;
                } else {
                    right_index -= 1;
                }
            }
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! saddleback_tests {
        ($($name:ident: $tc:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (matrix, element, expected) = $tc;
                    assert_eq!(saddleback_search(&matrix, element, true), expected);
                }
            )*
        };
    }

    saddleback_tests! {
        test_element_not_found: (
            vec![
                vec![1, 10, 100],
                vec![2, 20, 200],
                vec![3, 30, 300]
            ],
            123,
            Ok(None::<(usize, usize)>),
        ),
        test_element_at_top_left: (
            vec![
                vec![1, 10, 100],
                vec![2, 20, 200],
                vec![3, 30, 300]
            ],
            1,
            Ok(Some((0, 0))),
        ),
        test_element_at_bottom_right: (
            vec![
                vec![1, 10, 100],
                vec![2, 20, 200],
                vec![3, 30, 300]
            ],
            300,
            Ok(Some((2, 2))),
        ),
        test_element_at_top_right: (
            vec![
                vec![1, 10, 100],
                vec![2, 20, 200],
                vec![3, 30, 300]
            ],
            100,
            Ok(Some((0, 2))),
        ),
        test_element_at_bottom_left: (
            vec![
                vec![1, 10, 100],
                vec![2, 20, 200],
                vec![3, 30, 300]
            ],
            3,
            Ok(Some((2, 0))),
        ),
        test_element_in_middle: (
            vec![
                vec![1, 10, 100, 1000],
                vec![2, 20, 200, 2000],
                vec![3, 30, 300, 3000],
            ],
            200,
            Ok(Some((1, 2))),
        ),
        test_element_smaller_than_min: (
            vec![
                vec![1, 10, 100],
                vec![2, 20, 200],
                vec![3, 30, 300],
            ],
            0,
            Ok(None::<(usize, usize)>),
        ),
        test_horizontal: (
            vec![
                vec![1, 10, 100],
            ],
            100,
            Ok(Some((0, 2))),
        ),
        test_vertical: (
            vec![
                vec![1],
                vec![2],
                vec![3],
            ],
            2,
            Ok(Some((1, 0))),
        ),
        test_single_element: (
            vec![
                vec![1],
            ],
            1,
            Ok(Some((0, 0))),
        ),
        test_empty_matrix: (
            vec![],
            1,
            Ok(None::<(usize, usize)>),
        ),
        test_non_rectangular_matrix: (
            vec![
                vec![1, 10, 100],
                vec![2, 20],
                vec![3, 30, 300],
            ],
            20,
            Err::<Option<(usize, usize)>, MatrixError>(MatrixError::NonRectangularInput),
        ),
        test_empty_row: (
            vec![
                vec![1, 2, 3],
                vec![],
                vec![4, 5, 6],
            ],
            3,
            Err::<Option<(usize, usize)>, MatrixError>(MatrixError::NonRectangularInput),
        ),
        test_full_empty_rows: (
            vec![
                vec![],
                vec![],
                vec![],
                vec![],
            ],
            1,
            Ok(None::<(usize, usize)>),
        ),
        test_unsorted_matrix_with_check: (
            vec![
                vec![1, 10, 100],
                vec![20, 200, 2],
                vec![3, 30, 300],
            ],
            200,
            Err::<Option<(usize, usize)>, MatrixError>(MatrixError::NotSorted),
        ),
    }
}
