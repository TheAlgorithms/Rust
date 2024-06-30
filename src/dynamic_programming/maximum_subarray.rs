//! This module provides a function to find the subarray with the largest sum
//! in a given array of integers using dynamic programming. It also includes
//! tests to verify the correctness of the implementation.

/// Finds the subarray (containing at least one number) which has the largest sum
/// and returns its sum.
///
/// A subarray is a contiguous part of an array.
///
/// # Arguments
///
/// * `array` - A slice of integers.
///
/// # Returns
///
/// An integer representing the largest sum of a contiguous subarray.
///
/// # Complexity
///
/// * Time complexity: O(array.len())
/// * Space complexity: O(array.len())
pub fn maximum_subarray(array: &[isize]) -> isize {
    if array.is_empty() {
        return 0;
    }

    let mut max_sum = vec![0; array.len()];
    max_sum[0] = array[0];

    array.iter().enumerate().skip(1).for_each(|(i, &x)| {
        max_sum[i] = (max_sum[i - 1] + x).max(x);
    });

    *max_sum.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! maximum_subarray_tests {
        ($($name:ident: $tc:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (array, expected) = $tc;
                    assert_eq!(maximum_subarray(&array), expected);
                }
            )*
        }
    }

    maximum_subarray_tests! {
        test_all_non_negative: (vec![1, 0, 5, 8], 14),
        test_all_negative: (vec![-3, -1, -8, -2], -1),
        test_mixed_negative_and_positive: (vec![-4, 3, -2, 5, -8], 6),
        test_single_element_positive: (vec![6], 6),
        test_single_element_negative: (vec![-6], -6),
        test_mixed_elements: (vec![-2, 1, -3, 4, -1, 2, 1, -5, 4], 6),
        test_empty_array: (vec![], 0),
        test_all_zeroes: (vec![0, 0, 0, 0], 0),
        test_single_zero: (vec![0], 0),
        test_alternating_signs: (vec![3, -2, 5, -1], 6),
        test_all_negatives_with_one_positive: (vec![-3, -4, 1, -7, -2], 1),
        test_all_positives_with_one_negative: (vec![3, 4, -1, 7, 2], 15),
        test_all_positives: (vec![2, 3, 1, 5], 11),
        test_large_values: (vec![1000, -500, 1000, -500, 1000], 2000),
        test_large_array: ((0..1000).collect::<Vec<_>>(), 499500),
        test_large_negative_array: ((0..1000).map(|x| -x).collect::<Vec<_>>(), 0),
        test_single_large_positive: (vec![1000000], 1000000),
        test_single_large_negative: (vec![-1000000], -1000000),
    }
}
