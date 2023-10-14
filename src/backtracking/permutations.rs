/*
The permutations problem involves finding all possible permutations
of a given collection of distinct integers. For instance, given [1, 2, 3],
the goal is to generate permutations like
 [1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], and [3, 2, 1].
 This implementation uses a backtracking algorithm to generate all possible permutations.
*/

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new(); // Vector to store the resulting permutations
    let mut current_permutation = Vec::new(); // Vector to store the current permutation being constructed
    let mut used = vec![false; nums.len()]; // A boolean array to keep track of used elements

    backtrack(&nums, &mut current_permutation, &mut used, &mut result); // Call the backtracking function

    result // Return the list of permutations
}

fn backtrack(
    nums: &Vec<i32>,
    current_permutation: &mut Vec<i32>,
    used: &mut Vec<bool>,
    result: &mut Vec<Vec<i32>>,
) {
    if current_permutation.len() == nums.len() {
        // If the current permutation is of the same length as the input,
        // it is a complete permutation, so add it to the result.
        result.push(current_permutation.clone());
        return;
    }

    for i in 0..nums.len() {
        if used[i] {
            continue; // Skip used elements
        }

        current_permutation.push(nums[i]); // Add the current element to the permutation
        used[i] = true; // Mark the element as used

        backtrack(nums, current_permutation, used, result); // Recursively generate the next permutation

        current_permutation.pop(); // Backtrack by removing the last element
        used[i] = false; // Mark the element as unused for the next iteration
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        // Test case: Generate permutations for [1, 2, 3]
        let permutations = permute(vec![1, 2, 3]);

        assert_eq!(permutations.len(), 6); // There should be 6 permutations

        // Verification for some of the permutations
        assert!(permutations.contains(&vec![1, 2, 3]));
        assert!(permutations.contains(&vec![1, 3, 2]));
        assert!(permutations.contains(&vec![2, 1, 3]));
    }
}
