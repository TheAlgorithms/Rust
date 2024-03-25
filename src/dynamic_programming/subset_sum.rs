/// Determines whether there exists a subset of the given array `nums` that adds up to the specified `target_sum`.
/// If such a subset exists, returns the subset that adds up to `target_sum`, sorted in ascending order.
/// If no such subset exists, returns `None`.
///
/// # Arguments
///
/// * `nums` - A slice of integers representing the input array.
/// * `target_sum` - The target sum that the subset needs to achieve.
pub fn subset_sum(nums: &[u32], target_sum: u32) -> Option<Vec<u32>> {
    let n = nums.len();
    let mut dp = vec![vec![false; (target_sum + 1) as usize]; n + 1];

    // Base case: if the target sum is 0, then it's always possible to achieve it with an empty subset.
    for row in &mut dp {
        row[0] = true;
    }

    // Fill the dp table
    for (i, num) in nums.iter().enumerate() {
        for j in 1..=target_sum as usize {
            dp[i + 1][j] = dp[i][j];
            if *num <= j as u32 {
                dp[i + 1][j] |= dp[i][j - *num as usize];
            }
        }
    }

    // Reconstruct the subset
    let mut subset = Vec::new();
    let mut i = n;
    let mut j = target_sum as usize;
    while i > 0 && j > 0 {
        if dp[i][j] && !dp[i - 1][j] {
            subset.push(nums[i - 1]);
            j -= nums[i - 1] as usize;
        }
        i -= 1;
    }

    // Sort the subset array
    subset.sort();

    if dp[n][target_sum as usize] {
        Some(subset)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subset_sum_exists() {
        let nums = vec![3, 34, 4, 12, 5, 2];
        let target_sum = 9;
        let subset = subset_sum(&nums, target_sum);
        assert_eq!(subset, Some(vec![4, 5]));
    }

    #[test]
    fn test_subset_sum_does_not_exist() {
        let nums = vec![3, 34, 4, 12, 5, 2];
        let target_sum = 30; // No subset adds up to 50
        let subset = subset_sum(&nums, target_sum);
        assert_eq!(subset, None);
    }

    #[test]
    fn test_subset_sum_empty_array() {
        let nums = vec![];
        let target_sum = 5;
        let subset = subset_sum(&nums, target_sum);
        assert_eq!(subset, None);
    }

    #[test]
    fn test_subset_sum_single_element_array() {
        let nums = vec![7];
        let target_sum = 7;
        let subset = subset_sum(&nums, target_sum);
        assert_eq!(subset, Some(vec![7]));
    }
}
