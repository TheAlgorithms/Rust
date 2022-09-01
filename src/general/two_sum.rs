use std::collections::HashMap;

/// Given an array of integers nums and an integer target,
/// return indices of the two numbers such that they add up to target.
///
/// # Parameters
///
/// - `nums`: A list of numbers to check.
/// - `target`: The target sum.
///
/// # Returns
///
/// If the target sum is found in the array, the indices of the augend and
/// addend are returned as a tuple.
///
/// If the target sum cannot be found in the array, `None` is returned.
///
pub fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    // This HashMap is used to look up a corresponding index in the `nums` list.
    // Given that we know where we are at in the array, we can look up our
    // complementary value using this table and only go through the list once.
    //
    // We populate this table with distances from the target. As we go through
    // the list, a distance is computed like so:
    //
    //     `target - current_value`
    //
    // This distance also tells us about the complementary value we're looking
    // for in the array. If we don't find that value, we insert `current_value`
    // into the table for future look-ups. As we iterate through the list,
    // the number we just inserted might be the perfect distance for another
    // number, and we've found a match!
    //
    let mut distance_table: HashMap<i32, usize> = HashMap::new();

    for (i, current_value) in nums.iter().enumerate() {
        match distance_table.get(&(target - current_value)) {
            Some(j) => return Some((i, *j)),
            _ => distance_table.insert(*current_value, i),
        };
    }

    // No match was found!
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(two_sum(nums, 9), Some((1, 0)));

        let nums = vec![3, 2, 4];
        assert_eq!(two_sum(nums, 6), Some((2, 1)));

        let nums = vec![3, 3];
        assert_eq!(two_sum(nums, 6), Some((1, 0)));

        let nums = vec![2, 7, 11, 15];
        assert_eq!(two_sum(nums, 16), None);
    }
}
