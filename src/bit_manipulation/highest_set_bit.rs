//! This module provides a function to find the position of the most significant bit (MSB)
//! set to 1 in a given positive integer.

/// Finds the position of the highest (most significant) set bit in a positive integer.
///
/// # Arguments
///
/// * `num` - An integer value for which the highest set bit will be determined.
///
/// # Returns
///
/// *  Returns `Some(position)` if a set bit exists or `None` if no bit is set.
pub fn find_highest_set_bit(num: usize) -> Option<usize> {
    if num == 0 {
        return None;
    }

    let mut position = 0;
    let mut n = num;

    while n > 0 {
        n >>= 1;
        position += 1;
    }

    Some(position - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_find_highest_set_bit {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $test_case;
                    assert_eq!(find_highest_set_bit(input), expected);
                }
            )*
        };
    }

    test_find_highest_set_bit! {
        test_positive_number: (18, Some(4)),
        test_0: (0, None),
        test_1: (1, Some(0)),
        test_2: (2, Some(1)),
        test_3: (3, Some(1)),
    }
}
