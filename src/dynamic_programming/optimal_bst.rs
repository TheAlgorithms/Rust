// Optimal Binary Search Tree Algorithm in Rust
// Time Complexity: O(n^3) with prefix sum optimization
// Space Complexity: O(n^2) for the dp table and prefix sum array

/// Constructs an Optimal Binary Search Tree from a list of key frequencies.
/// The goal is to minimize the expected search cost given key access frequencies.
///
/// # Arguments
/// * `freq` - A slice of integers representing the frequency of key access
///
/// # Returns
/// * An integer representing the minimum cost of the optimal BST
pub fn optimal_search_tree(freq: &[i32]) -> i32 {
    let n = freq.len();
    if n == 0 {
        return 0;
    }

    // dp[i][j] stores the cost of optimal BST that can be formed from keys[i..=j]
    let mut dp = vec![vec![0; n]; n];

    // prefix_sum[i] stores sum of freq[0..i]
    let mut prefix_sum = vec![0; n + 1];
    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + freq[i];
    }

    // Base case: Trees with only one key
    for i in 0..n {
        dp[i][i] = freq[i];
    }

    // Build chains of increasing length l (from 2 to n)
    for l in 2..=n {
        for i in 0..=n - l {
            let j = i + l - 1;
            dp[i][j] = i32::MAX;

            // Compute the total frequency sum in the range [i..=j] using prefix sum
            let fsum = prefix_sum[j + 1] - prefix_sum[i];

            // Try making each key in freq[i..=j] the root of the tree
            for r in i..=j {
                // Cost of left subtree
                let left = if r > i { dp[i][r - 1] } else { 0 };
                // Cost of right subtree
                let right = if r < j { dp[r + 1][j] } else { 0 };

                // Total cost = left + right + sum of frequencies (fsum)
                let cost = left + right + fsum;

                // Choose the minimum among all possible roots
                if cost < dp[i][j] {
                    dp[i][j] = cost;
                }
            }
        }
    }

    // Minimum cost of the optimal BST storing all keys
    dp[0][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    // Macro to generate multiple test cases for the optimal_search_tree function
    macro_rules! optimal_bst_tests {
        ($($name:ident: $input:expr => $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let freq = $input;
                    assert_eq!(optimal_search_tree(freq), $expected);
                }
            )*
        };
    }

    optimal_bst_tests! {
        // Common test cases
        test_case_1: &[34, 10, 8, 50] => 180,
        test_case_2: &[10, 12] => 32,
        test_case_3: &[10, 12, 20] => 72,
        test_case_4: &[25, 10, 20] => 95,
        test_case_5: &[4, 2, 6, 3] => 26,

        // Edge test cases
        test_case_single: &[42] => 42,
        test_case_empty: &[] => 0,
    }
}
