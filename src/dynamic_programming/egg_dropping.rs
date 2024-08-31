//! This module contains the `egg_drop` function, which determines the minimum number of egg droppings
//! required to find the highest floor from which an egg can be dropped without breaking. It also includes
//! tests for the function using various test cases, including edge cases.

/// Returns the least number of egg droppings required to determine the highest floor from which an egg will not break upon dropping.
///
/// # Arguments
///
/// * `eggs` - The number of eggs available.
/// * `floors` - The number of floors in the building.
///
/// # Returns
///
/// * `Some(usize)` - The minimum number of drops required if the number of eggs is greater than 0.
/// * `None` - If the number of eggs is 0.
pub fn egg_drop(eggs: usize, floors: usize) -> Option<usize> {
    if eggs == 0 {
        return None;
    }

    if eggs == 1 || floors == 0 || floors == 1 {
        return Some(floors);
    }

    // Create a 2D vector to store solutions to subproblems
    let mut egg_drops: Vec<Vec<usize>> = vec![vec![0; floors + 1]; eggs + 1];

    // Base cases: 0 floors -> 0 drops, 1 floor -> 1 drop
    (1..=eggs).for_each(|i| {
        egg_drops[i][1] = 1;
    });

    // Base case: 1 egg -> k drops for k floors
    (1..=floors).for_each(|j| {
        egg_drops[1][j] = j;
    });

    // Fill the table using the optimal substructure property
    (2..=eggs).for_each(|i| {
        (2..=floors).for_each(|j| {
            egg_drops[i][j] = (1..=j)
                .map(|k| 1 + std::cmp::max(egg_drops[i - 1][k - 1], egg_drops[i][j - k]))
                .min()
                .unwrap();
        });
    });

    Some(egg_drops[eggs][floors])
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! egg_drop_tests {
        ($($name:ident: $test_cases:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (eggs, floors, expected) = $test_cases;
                    assert_eq!(egg_drop(eggs, floors), expected);
                }
            )*
        }
    }

    egg_drop_tests! {
        test_no_floors: (5, 0, Some(0)),
        test_one_egg_multiple_floors: (1, 8, Some(8)),
        test_multiple_eggs_one_floor: (5, 1, Some(1)),
        test_two_eggs_two_floors: (2, 2, Some(2)),
        test_three_eggs_five_floors: (3, 5, Some(3)),
        test_two_eggs_ten_floors: (2, 10, Some(4)),
        test_two_eggs_thirty_six_floors: (2, 36, Some(8)),
        test_many_eggs_one_floor: (100, 1, Some(1)),
        test_many_eggs_few_floors: (100, 5, Some(3)),
        test_few_eggs_many_floors: (2, 1000, Some(45)),
        test_zero_eggs: (0, 10, None::<usize>),
        test_no_eggs_no_floors: (0, 0, None::<usize>),
        test_one_egg_no_floors: (1, 0, Some(0)),
        test_one_egg_one_floor: (1, 1, Some(1)),
        test_maximum_floors_one_egg: (1, usize::MAX, Some(usize::MAX)),
    }
}
