//! This module provides functionality to check if there exists a subset of a given set of integers
//! that sums to a target value. The implementation uses a recursive backtracking approach.

/// Checks if there exists a subset of the given set that sums to the target value.
pub fn has_subset_with_sum(set: &[isize], target: isize) -> bool {
    backtrack(set, set.len(), target)
}

fn backtrack(set: &[isize], remaining_items: usize, target: isize) -> bool {
    // Found a subset with the required sum
    if target == 0 {
        return true;
    }
    // No more elements to process
    if remaining_items == 0 {
        return false;
    }
    // Check if we can find a subset including or excluding the last element
    backtrack(set, remaining_items - 1, target)
        || backtrack(set, remaining_items - 1, target - set[remaining_items - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! has_subset_with_sum_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (set, target, expected) = $test_case;
                    assert_eq!(has_subset_with_sum(set, target), expected);
                }
            )*
        }
    }

    has_subset_with_sum_tests! {
        test_small_set_with_sum: (&[3, 34, 4, 12, 5, 2], 9, true),
        test_small_set_without_sum: (&[3, 34, 4, 12, 5, 2], 30, false),
        test_consecutive_set_with_sum: (&[1, 2, 3, 4, 5, 6], 10, true),
        test_consecutive_set_without_sum: (&[1, 2, 3, 4, 5, 6], 22, false),
        test_large_set_with_sum: (&[5, 10, 12, 13, 15, 18, -1, 10, 50, -2, 3, 4], 30, true),
        test_empty_set: (&[], 0, true),
        test_empty_set_with_nonzero_sum: (&[], 10, false),
        test_single_element_equal_to_sum: (&[10], 10, true),
        test_single_element_not_equal_to_sum: (&[5], 10, false),
        test_negative_set_with_sum: (&[-7, -3, -2, 5, 8], 0, true),
        test_negative_sum: (&[1, 2, 3, 4, 5], -1, false),
        test_negative_sum_with_negatives: (&[-7, -3, -2, 5, 8], -4, true),
        test_negative_sum_with_negatives_no_solution: (&[-7, -3, -2, 5, 8], -14, false),
        test_even_inputs_odd_target: (&[2, 4, 6, 2, 8, -2, 10, 12, -24, 8, 12, 18], 3, false),
    }
}
