//! This module provides a function to add two integers without using the `+` operator.
//! It relies on bitwise operations (XOR and AND) to compute the sum, simulating the addition process.

/// Adds two integers using bitwise operations.
///
/// # Arguments
///
/// * `a` - The first integer to be added.
/// * `b` - The second integer to be added.
///
/// # Returns
///
/// * `isize` - The result of adding the two integers.
pub fn add_two_integers(mut a: isize, mut b: isize) -> isize {
    let mut carry;

    while b != 0 {
        let sum = a ^ b;
        carry = (a & b) << 1;
        a = sum;
        b = carry;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_add_two_integers {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (a, b) = $test_case;
                    assert_eq!(add_two_integers(a, b), a + b);
                    assert_eq!(add_two_integers(b, a), a + b);
                }
            )*
        };
    }

    test_add_two_integers! {
        test_add_two_integers_positive: (3, 5),
        test_add_two_integers_large_positive: (100, 200),
        test_add_two_integers_edge_positive: (65535, 1),
        test_add_two_integers_negative: (-10, 6),
        test_add_two_integers_both_negative: (-50, -30),
        test_add_two_integers_edge_negative: (-1, -1),
        test_add_two_integers_zero: (0, 0),
        test_add_two_integers_zero_with_positive: (0, 42),
        test_add_two_integers_zero_with_negative: (0, -42),
    }
}
