//! This module provides a function to generate all possible permutations
//! of a given collection of distinct integers using a backtracking algorithm.

/// Generates all possible permutations of a given vector of distinct integers.
///
/// # Arguments
///
/// * `nums` - A vector of distinct integers.
///
/// # Returns
///
/// A vector containing all possible permutations of the input vector.
pub fn permute(nums: Vec<isize>) -> Vec<Vec<isize>> {
    let mut permutations = Vec::new();
    let mut current = Vec::new();
    let mut used = vec![false; nums.len()];

    generate(&nums, &mut current, &mut used, &mut permutations);

    permutations
}

/// Helper function for the `permute` function to generate permutations recursively.
///
/// # Arguments
///
/// * `nums` - A reference to the original vector of distinct integers.
/// * `current` - A mutable reference to the vector holding the current permutation.
/// * `used` - A mutable reference to a vector tracking which elements are used.
/// * `permutations` - A mutable reference to the vector holding all generated permutations.
fn generate(
    nums: &Vec<isize>,
    current: &mut Vec<isize>,
    used: &mut Vec<bool>,
    permutations: &mut Vec<Vec<isize>>,
) {
    if current.len() == nums.len() {
        permutations.push(current.clone());
        return;
    }

    for idx in 0..nums.len() {
        if used[idx] {
            continue;
        }

        current.push(nums[idx]);
        used[idx] = true;

        generate(nums, current, used, permutations);

        current.pop();
        used[idx] = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! permute_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $test_case;
                    assert_eq!(permute(input), expected);
                }
            )*
        }
    }

    permute_tests! {
        test_permute_123: (vec![1, 2, 3], vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ]),
        test_permute_empty: (Vec::<isize>::new(), vec![vec![]]),
        test_permute_single: (vec![1], vec![vec![1]]),
        test_permute_duplicates: (vec![1, 1, 2], vec![
            vec![1, 1, 2],
            vec![1, 2, 1],
            vec![1, 1, 2],
            vec![1, 2, 1],
            vec![2, 1, 1],
            vec![2, 1, 1],
        ]),
        test_permute_negative: (vec![-1, -2, -3], vec![
            vec![-1, -2, -3],
            vec![-1, -3, -2],
            vec![-2, -1, -3],
            vec![-2, -3, -1],
            vec![-3, -1, -2],
            vec![-3, -2, -1],
        ]),
        test_permute_mixed: (vec![-1, 0, 1], vec![
            vec![-1, 0, 1],
            vec![-1, 1, 0],
            vec![0, -1, 1],
            vec![0, 1, -1],
            vec![1, -1, 0],
            vec![1, 0, -1],
        ]),
        test_permute_larger: (vec![1, 2, 3, 4], vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 4, 3],
            vec![1, 3, 2, 4],
            vec![1, 3, 4, 2],
            vec![1, 4, 2, 3],
            vec![1, 4, 3, 2],
            vec![2, 1, 3, 4],
            vec![2, 1, 4, 3],
            vec![2, 3, 1, 4],
            vec![2, 3, 4, 1],
            vec![2, 4, 1, 3],
            vec![2, 4, 3, 1],
            vec![3, 1, 2, 4],
            vec![3, 1, 4, 2],
            vec![3, 2, 1, 4],
            vec![3, 2, 4, 1],
            vec![3, 4, 1, 2],
            vec![3, 4, 2, 1],
            vec![4, 1, 2, 3],
            vec![4, 1, 3, 2],
            vec![4, 2, 1, 3],
            vec![4, 2, 3, 1],
            vec![4, 3, 1, 2],
            vec![4, 3, 2, 1],
        ]),
    }
}
