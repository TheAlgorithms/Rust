/**
 * @file
 * @brief Find the maximum subarray sum using Kadane's algorithm.(https://en.wikipedia.org/wiki/Maximum_subarray_problem)
 *
 * @details
 * This program provides a function to find the maximum subarray sum in an array of integers
 * using Kadane's algorithm.
 *
 * @param arr A slice of integers representing the array.
 * @return The maximum subarray sum.
 *
 * @author [Gyandeep] (https://github.com/Gyan172004)
 * @see Wikipedia - Maximum subarray problem
 */

/**
 * Find the maximum subarray sum using Kadane's algorithm.
 * @param arr A slice of integers representing the array.
 * @return The maximum subarray sum.
 */
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut max_current = nums[0];
    let mut max_global = nums[0];

    nums.iter().skip(1).for_each(|&item| {
        max_current = std::cmp::max(item, max_current + item);
        if max_current > max_global {
            max_global = max_current;
        }
    });
    max_global
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Test case for Kadane's algorithm with positive numbers.
     */
    #[test]
    fn test_kadanes_algorithm_positive() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(max_sub_array(arr.to_vec()), 15);
    }

    /**
     * Test case for Kadane's algorithm with negative numbers.
     */
    #[test]
    fn test_kadanes_algorithm_negative() {
        let arr = [-2, -3, -4, -1, -2];
        assert_eq!(max_sub_array(arr.to_vec()), -1);
    }

    /**
     * Test case for Kadane's algorithm with mixed numbers.
     */
    #[test]
    fn test_kadanes_algorithm_mixed() {
        let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(max_sub_array(arr.to_vec()), 6);
    }

    /**
     * Test case for Kadane's algorithm with an empty array.
     */
    #[test]
    fn test_kadanes_algorithm_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(max_sub_array(arr.to_vec()), 0);
    }

    /**
     * Test case for Kadane's algorithm with a single positive number.
     */
    #[test]
    fn test_kadanes_algorithm_single_positive() {
        let arr = [10];
        assert_eq!(max_sub_array(arr.to_vec()), 10);
    }
}
