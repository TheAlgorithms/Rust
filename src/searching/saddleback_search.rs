use std::cmp::Ordering;

/// Custom error type to represent errors related to matrix validation.
#[derive(Debug, PartialEq, Eq)]
pub enum MatrixError {
    NonRectangularMatrix,
}

/// Performs Saddleback search on a sorted 2D matrix.
///
/// The Saddleback search algorithm finds the position of a target element in a matrix where
/// each row and each column is sorted in ascending order. The search starts from the top-right
/// corner of the matrix and moves left or down based on comparisons with the target element.
///
/// # Arguments
///
/// * `matrix` - A 2D vector representing the sorted matrix.
/// * `element` - The target element to search for.
///
/// # Returns
///
/// Returns `Ok(Some((row, column)))` where both indices are 0-based if the element is found.
/// Returns `Ok(None)` if the element is not found.
/// Returns `Err(MatrixError)` if the matrix is not rectangular.
pub fn saddleback_search(
    matrix: &[Vec<isize>],
    element: isize,
) -> Result<Option<(usize, usize)>, MatrixError> {
    if matrix.is_empty() || matrix.iter().all(|row| row.is_empty()) {
        return Ok(None);
    }

    if matrix.iter().any(|row| row.len() != matrix[0].len()) {
        return Err(MatrixError::NonRectangularMatrix);
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
                    assert_eq!(saddleback_search(&matrix, element), expected);
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
            Err::<Option<(usize, usize)>, MatrixError>(MatrixError::NonRectangularMatrix),
        ),
        test_empty_row: (
            vec![
                vec![1, 2, 3],
                vec![],
                vec![4, 5, 6],
            ],
            3,
            Err::<Option<(usize, usize)>, MatrixError>(MatrixError::NonRectangularMatrix),
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
    }
}
