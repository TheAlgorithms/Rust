// Subset Sum Problem in Rust
// Time Complexity: O(n * sum) where n is array length and sum is the target sum
// Space Complexity: O(n * sum) for the DP table
/// Determines if there exists a subset of the given array that sums to the target value.
/// Uses dynamic programming to solve the subset sum problem.
///
/// # Arguments
/// * `arr` - A slice of integers representing the input array.
/// * `required_sum` - The target sum to check for.
///
/// # Returns
/// * `bool` - A boolean indicating whether a subset exists that sums to the target.
pub fn is_sum_subset(arr: &[i32], required_sum: i32) -> bool {
    let n = arr.len();

    // Handle edge case where required sum is 0 (empty subset always sums to 0)
    if required_sum == 0 {
        return true;
    }

    // Handle edge case where array is empty but required sum is positive
    if n == 0 && required_sum > 0 {
        return false;
    }
    // dp[i][j] stores whether sum j can be achieved using first i elements
    let mut dp = vec![vec![false; required_sum as usize + 1]; n + 1];
    // Base case: sum 0 can always be achieved with any number of elements (empty subset)
    for i in 0..=n {
        dp[i][0] = true;
    }
    // Base case: with 0 elements, no positive sum can be achieved
    for j in 1..=required_sum as usize {
        dp[0][j] = false;
    }
    // Fill the DP table
    for i in 1..=n {
        for j in 1..=required_sum as usize {
            if arr[i - 1] > j as i32 {
                // Current element is too large, exclude it
                dp[i][j] = dp[i - 1][j];
            } else {
                // Either exclude the current element or include it
                dp[i][j] = dp[i - 1][j] || dp[i - 1][j - arr[i - 1] as usize];
            }
        }
    }
    dp[n][required_sum as usize]
}
#[cfg(test)]
mod tests {
    use super::*;
    // Macro to generate multiple test cases for the is_sum_subset function
    macro_rules! subset_sum_tests {
        ($($name:ident: $input:expr => $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (arr, sum) = $input;
                    assert_eq!(is_sum_subset(arr, sum), $expected);
                }
            )*
        };
    }
    subset_sum_tests! {
        // Common test cases
        test_case_1: (&[2, 4, 6, 8], 5) => false,
        test_case_2: (&[2, 4, 6, 8], 14) => true,
        test_case_3: (&[3, 34, 4, 12, 5, 2], 9) => true,
        test_case_4: (&[3, 34, 4, 12, 5, 2], 30) => false,
        test_case_5: (&[1, 2, 3, 4, 5], 15) => true,

        // Edge test cases
        test_case_empty_array_positive_sum: (&[], 5) => false,
        test_case_empty_array_zero_sum: (&[], 0) => true,
        test_case_zero_sum: (&[1, 2, 3], 0) => true,
        test_case_single_element_match: (&[5], 5) => true,
        test_case_single_element_no_match: (&[3], 5) => false,
    }
}
