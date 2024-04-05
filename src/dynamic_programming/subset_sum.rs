/// Computes subsets of a given array of numbers that sum up to a target sum.
///
/// # Arguments
///
/// * `nums` - A slice of unsigned 32-bit integers representing the input numbers.
/// * `target_sum` - The target sum to which subsets are sought.
///
/// # Returns
///
/// An option containing a vector of vectors representing subsets that sum up to the target sum,
/// or `None` if no such subsets exist.
pub fn subset_sum(nums: &[u32], target_sum: u32) -> Option<Vec<Vec<u32>>> {
    let mut dp = initialize_dp_table(nums.len(), target_sum);

    fill_dp_table(nums, &mut dp);

    extract_subsets(&dp, nums.len(), target_sum)
}

/// Generates a dynamic programming table for the subset sum problem.
///
/// # Arguments
///
/// * `nums` - A slice of unsigned 32-bit integers representing the input numbers.
/// * `target_sum` - The target sum to which subsets are sought.
///
/// # Returns
///
/// A dynamic programming table containing subsets that sum up to specific values.
fn generate_subset_table(nums: &[u32], target_sum: u32) -> Vec<Vec<Vec<Vec<u32>>>> {
    let mut subset_table = vec![vec![vec![]; (target_sum + 1) as usize]; nums.len() + 1];

    // Base case: Empty subset is a valid solution for target sum 0
    for row in &mut subset_table {
        row[0] = vec![vec![]];
    }

    // Fill subset_table table
    for (i, &num) in nums.iter().enumerate() {
        for j in 1..=target_sum {
            let mut new_subsets = subset_table[i][j as usize].clone();
            if num <= j {
                let prev_subsets = &subset_table[i][(j - num) as usize];
                for prev_subset in prev_subsets {
                    let mut new_subset = prev_subset.clone();
                    new_subset.push(num);
                    new_subsets.push(new_subset);
                }
            }
            for subset in new_subsets {
                subset_table[i + 1][j as usize].push(subset);
            }
        }
    }

    subset_table
}

/// Extracts subsets that sum up to the target sum from the subset table.
///
/// # Arguments
///
/// * `dp` - A reference to the DP table.
/// * `i` - Index representing the current number being considered.
/// * `num` - The current number being considered.
/// * `j` - The target sum for which subsets are sought.
///
/// # Returns
///
/// A vector of vectors representing subsets that sum up to the target sum.
fn update_subsets(dp: &[Vec<Vec<Vec<u32>>>], i: usize, num: u32, j: u32) -> Vec<Vec<u32>> {
    let mut new_subsets = dp[i][j as usize].clone();

    // If the current number is less than or equal to the target sum,
    // update subsets with the current number
    if num <= j {
        let prev_subsets = dp[i][(j - num) as usize].clone();
        for prev_subset in prev_subsets {
            let mut new_subset = prev_subset.clone();
            new_subset.push(num);
            new_subsets.push(new_subset);
        }
    }

    new_subsets
}

/// Extracts subsets that sum up to the target sum from the DP table.
///
/// # Arguments
///
/// * `dp` - A reference to the DP table.
/// * `rows` - The number of rows in the DP table.
/// * `columns` - The number of columns in the DP table.
///
/// # Returns
///
/// An option containing a vector of vectors representing subsets that sum up to the target sum,
/// or `None` if no such subsets exist.
fn extract_subsets(dp: &[Vec<Vec<Vec<u32>>>], rows: usize, columns: u32) -> Option<Vec<Vec<u32>>> {
    let mut result = dp[rows][columns as usize].clone();
    // Sort subsets to make duplicates adjacent
    result.sort();
    // Remove consecutive duplicates
    result.dedup();
    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_subset_sum {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (nums, target_sum, expected) = $inputs;
                    assert_eq!(subset_sum(&nums, target_sum), expected);
                }
            )*
        }
    }

    test_subset_sum! {
        test_subset_sum_basic: (vec![1, 2, 3, 4, 5], 5, Some(vec![vec![1, 4], vec![2, 3], vec![5]])),
        test_subset_sum_empty_input: (vec![], 10, None),
        test_subset_sum_no_solution: (vec![1, 2, 3, 4, 5], 100, None),
        test_subset_sum_single_element: (vec![7], 7, Some(vec![vec![7]])),
        test_subset_sum_with_duplicates: (vec![1, 2, 2, 3], 5, Some(vec![vec![1, 2, 2], vec![2, 3]])),
        test_subset_sum_with_non_unique_elements: (vec![1, 2, 3, 3, 4], 7, Some(vec![vec![1, 2, 4], vec![1, 3, 3], vec![3, 4]])),
    }
}
