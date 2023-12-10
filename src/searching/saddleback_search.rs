// Saddleback search is a technique used to find an element in a sorted 2D matrix in O(m + n) time,
// where m is the number of rows, and n is the number of columns. It works by starting from the
// top-right corner of the matrix and moving left or down based on the comparison of the current
// element with the target element.
use std::cmp::Ordering;

pub fn saddleback_search(matrix: &[Vec<i32>], element: i32) -> (usize, usize) {
    // Initialize left and right indices
    let mut left_index = 0;
    let mut right_index = matrix[0].len() - 1;

    // Start searching
    while left_index < matrix.len() {
        match element.cmp(&matrix[left_index][right_index]) {
            // If the current element matches the target element, return its position (indices are 1-based)
            Ordering::Equal => return (left_index + 1, right_index + 1),
            Ordering::Greater => {
                // If the target element is greater, move to the next row (downwards)
                left_index += 1;
            }
            Ordering::Less => {
                // If the target element is smaller, move to the previous column (leftwards)
                if right_index == 0 {
                    break; // If we reach the left-most column, exit the loop
                } else {
                    right_index -= 1;
                }
            }
        }
    }

    // If the element is not found, return (0, 0)
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test when the element is not present in the matrix
    #[test]
    fn test_element_not_found() {
        let matrix = vec![vec![1, 10, 100], vec![2, 20, 200], vec![3, 30, 300]];
        assert_eq!(saddleback_search(&matrix, 123), (0, 0));
    }

    // Test when the element is at the top-left corner of the matrix
    #[test]
    fn test_element_at_top_left() {
        let matrix = vec![vec![1, 10, 100], vec![2, 20, 200], vec![3, 30, 300]];
        assert_eq!(saddleback_search(&matrix, 1), (1, 1));
    }

    // Test when the element is at the bottom-right corner of the matrix
    #[test]
    fn test_element_at_bottom_right() {
        let matrix = vec![vec![1, 10, 100], vec![2, 20, 200], vec![3, 30, 300]];
        assert_eq!(saddleback_search(&matrix, 300), (3, 3));
    }

    // Test when the element is at the top-right corner of the matrix
    #[test]
    fn test_element_at_top_right() {
        let matrix = vec![vec![1, 10, 100], vec![2, 20, 200], vec![3, 30, 300]];
        assert_eq!(saddleback_search(&matrix, 100), (1, 3));
    }

    // Test when the element is at the bottom-left corner of the matrix
    #[test]
    fn test_element_at_bottom_left() {
        let matrix = vec![vec![1, 10, 100], vec![2, 20, 200], vec![3, 30, 300]];
        assert_eq!(saddleback_search(&matrix, 3), (3, 1));
    }

    // Additional test case: Element in the middle of the matrix
    #[test]
    fn test_element_in_middle() {
        let matrix = vec![
            vec![1, 10, 100, 1000],
            vec![2, 20, 200, 2000],
            vec![3, 30, 300, 3000],
        ];
        assert_eq!(saddleback_search(&matrix, 200), (2, 3));
    }
}
