// Check if a number is a perfect cube using binary search.
pub fn perfect_cube_binary_search(n: i64) -> bool {
    if n < 0 {
        return perfect_cube_binary_search(-n);
    }

    // Initialize left and right boundaries for binary search.
    let mut left = 0;
    let mut right = n.abs(); // Use the absolute value to handle negative numbers

    // Binary search loop to find the cube root.
    while left <= right {
        // Calculate the mid-point.
        let mid = left + (right - left) / 2;
        // Calculate the cube of the mid-point.
        let cube = mid * mid * mid;

        // Check if the cube equals the original number.
        match cube.cmp(&n) {
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid - 1,
        }
    }

    // If no cube root is found, return false.
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_perfect_cube {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (n, expected) = $inputs;
                assert_eq!(perfect_cube_binary_search(n), expected);
                assert_eq!(perfect_cube_binary_search(-n), expected);
            }
        )*
        }
    }

    test_perfect_cube! {
        num_0_a_perfect_cube: (0, true),
        num_1_is_a_perfect_cube: (1, true),
        num_27_is_a_perfect_cube: (27, true),
        num_64_is_a_perfect_cube: (64, true),
        num_8_is_a_perfect_cube: (8, true),
        num_2_is_not_a_perfect_cube: (2, false),
        num_3_is_not_a_perfect_cube: (3, false),
        num_4_is_not_a_perfect_cube: (4, false),
        num_5_is_not_a_perfect_cube: (5, false),
        num_999_is_not_a_perfect_cube: (999, false),
        num_1000_is_a_perfect_cube: (1000, true),
        num_1001_is_not_a_perfect_cube: (1001, false),
    }
}
