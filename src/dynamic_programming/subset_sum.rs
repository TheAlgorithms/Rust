/// Computes subsets of a given array of numbers that sum up to a target sum.
///
/// # Arguments
///
/// * `nums` - A slice of unsigned 32-bit integers representing the input numbers.
/// * `target_sum` - The target sum to which subsets are sought.
///
/// # Returns
///
/// An option containing a vector of vectors representing indices of elements in the input `nums`
/// vector that form subsets summing up to the target sum, or `None` if no such subsets exist.
pub fn find_subsets_with_sum(nums: &[u32], target_sum: u32) -> Option<Vec<Vec<usize>>> {
    let subset_table = generate_subset_table(nums, target_sum);
    extract_subset_indices(&subset_table, nums.len(), target_sum, nums)
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
/// * `subset_table` - A reference to the subset table.
/// * `rows` - The number of rows in the subset table.
/// * `columns` - The number of columns in the subset table.
/// * `nums` - A slice of unsigned 32-bit integers representing the input numbers.
///
/// # Returns
///
/// An option containing a vector of vectors representing indices of elements in the input `nums`
/// vector that form subsets summing up to the target sum, or `None` if no such subsets exist.
fn extract_subset_indices(
    subset_table: &[Vec<Vec<Vec<u32>>>],
    rows: usize,
    columns: u32,
    nums: &[u32],
) -> Option<Vec<Vec<usize>>> {
    let subsets = &subset_table[rows][columns as usize];

    if subsets.is_empty() {
        return None;
    }

    let mut result = Vec::new();

    // Extract indices of nums in the subsets
    for subset in subsets {
        let mut indices = Vec::new();
        for &num in subset {
            if let Some(index) = nums.iter().position(|&x| x == num) {
                indices.push(index);
            }
        }
        // Sort indices
        indices.sort();
        result.push(indices);
    }

    // Sort vector of vectors
    result.sort();
    // Remove consecutive duplicates
    result.dedup();

    Some(result)
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
                    assert_eq!(find_subsets_with_sum(&nums, target_sum), expected);
                }
            )*
        }
    }

    test_subset_sum! {
        test_subset_sum_basic: (vec![5, 1, 3, 4, 2], 5, Some(vec![vec![0], vec![1, 3], vec![2, 4]])),
        test_subset_sum_empty_input: (vec![], 10, None),
        test_subset_sum_no_solution: (vec![1, 3, 2, 5, 4], 100, None),
        test_subset_sum_single_element: (vec![7], 7, Some(vec![vec![0]])),
        test_subset_sum_with_duplicates: (vec![1, 3, 2, 2], 5, Some(vec![vec![0, 2, 2], vec![1, 2]])),
        test_subset_sum_with_non_unique_elements: (vec![1, 3, 2, 4, 3], 7, Some(vec![vec![0, 1, 1], vec![0, 2, 3], vec![1, 3]])),
    }
}
