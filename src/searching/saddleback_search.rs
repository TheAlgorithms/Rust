use std::cmp::Ordering;

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
/// Returns `Some((row, column))` where both indices are 0-based. If the element is not found, returns `None`.
pub fn saddleback_search(matrix: &[Vec<isize>], element: isize) -> Option<(usize, usize)> {
    let mut left_index = 0;
    let mut right_index = matrix[0].len() - 1;

    while left_index < matrix.len() {
        match element.cmp(&matrix[left_index][right_index]) {
            Ordering::Equal => return Some((left_index, right_index)),
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

    None
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
                    if let Some(expected_pos) = expected {
                        assert_eq!(matrix[expected_pos.0][expected_pos.1], element);
                    }
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
            None::<(usize, usize)>,
        ),
        test_element_at_top_left: (
            vec![
                vec![1, 10, 100],
                vec![2, 20, 200],
                vec![3, 30, 300]
            ],
            1,
            Some((0, 0)),
        ),
        test_element_at_bottom_right: (
            vec![
                vec![1, 10, 100],
                vec![2, 20, 200],
                vec![3, 30, 300]
            ],
            300,
            Some((2, 2)),
        ),
        test_element_at_top_right: (
            vec![
                vec![1, 10, 100],
                vec![2, 20, 200],
                vec![3, 30, 300]
            ],
            100,
            Some((0, 2)),
        ),
        test_element_at_bottom_left: (
            vec![
                vec![1, 10, 100],
                vec![2, 20, 200],
                vec![3, 30, 300]
            ],
            3,
            Some((2, 0)),
        ),
        test_element_in_middle: (
            vec![
                vec![1, 10, 100, 1000],
                vec![2, 20, 200, 2000],
                vec![3, 30, 300, 3000],
            ],
            200,
            Some((1, 2)),
        ),
        test_element_smaller_than_min: (
            vec![
                vec![1, 10, 100],
                vec![2, 20, 200],
                vec![3, 30, 300],
            ],
            0,
            None::<(usize, usize)>,
        ),
        test_horizontal: (
            vec![
                vec![1, 10, 100],
            ],
            100,
            Some((0, 2)),
        ),
        test_vertical: (
            vec![
                vec![1],
                vec![2],
                vec![3],
            ],
            2,
            Some((1, 0)),
        ),
        test_single_element: (
            vec![
                vec![1],
            ],
            1,
            Some((0, 0)),
        ),
    }
}
