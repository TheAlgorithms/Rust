//! This module provides a function to generate all possible combinations
//! of `k` numbers out of `1...n` using a backtracking algorithm.

/// Generates all possible combinations of `k` numbers out of `1...n`.
///
/// # Arguments
///
/// * `n` - The upper limit of the range (1 to `n`).
/// * `k` - The number of elements in each combination.
///
/// # Returns
///
/// A vector containing all possible combinations of `k` numbers out of `1...n`.
pub fn generate_all_combinations(n: isize, k: isize) -> Vec<Vec<isize>> {
    let mut combinations = vec![];
    backtrack(1, n, k, &mut vec![], &mut combinations);
    combinations
}

/// Helper function to generate combinations recursively.
///
/// # Arguments
///
/// * `start` - The current number to start the combination with.
/// * `n` - The upper limit of the range (1 to `n`).
/// * `k` - The number of elements left to complete the combination.
/// * `current` - A mutable reference to the current combination being constructed.
/// * `combinations` - A mutable reference to the vector holding all combinations.
fn backtrack(
    start: isize,
    n: isize,
    k: isize,
    current: &mut Vec<isize>,
    combinations: &mut Vec<Vec<isize>>,
) {
    if k == 0 {
        combinations.push(current.clone());
        return;
    }

    for num in start..=(n - k + 1) {
        current.push(num);
        backtrack(num + 1, n, k - 1, current, combinations);
        current.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! combination_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (n, k, expected) = $test_case;
                    assert_eq!(generate_all_combinations(n, k), expected);
                }
            )*
        }
    }

    combination_tests! {
        test_generate_4_2: (4, 2, vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ]),
        test_generate_4_3: (4, 3, vec![
            vec![1, 2, 3],
            vec![1, 2, 4],
            vec![1, 3, 4],
            vec![2, 3, 4],
        ]),
        test_generate_5_3: (5, 3, vec![
            vec![1, 2, 3],
            vec![1, 2, 4],
            vec![1, 2, 5],
            vec![1, 3, 4],
            vec![1, 3, 5],
            vec![1, 4, 5],
            vec![2, 3, 4],
            vec![2, 3, 5],
            vec![2, 4, 5],
            vec![3, 4, 5],
        ]),
        test_generate_5_1: (5, 1, vec![
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![5],
        ]),
        test_empty: (0, 0, vec![vec![]]),
        test_generate_n_eq_k: (3, 3, vec![
            vec![1, 2, 3],
        ]),
        test_generate_k_greater_than_n: (3, 4, Vec::<Vec<isize>>::new()),
    }
}
