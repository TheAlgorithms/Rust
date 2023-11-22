/// Aliquot sum of a number is defined as the sum of the proper divisors of a number.\
/// i.e. all the divisors of a number apart from the number itself.
///
/// ## Example:
/// The aliquot sum of 6 is (1 + 2 + 3) = 6, and that of 15 is (1 + 3 + 5) = 9
///
/// Wikipedia article on Aliquot Sum: <https://en.wikipedia.org/wiki/Aliquot_sum>

pub fn aliquot_sum(number: u64) -> u64 {
    if number == 0 {
        panic!("Input has to be positive.")
    }

    (1..=number / 2).filter(|&d| number % d == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_aliquot_sum {
        ($($name:ident: $tc:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (number, expected) = $tc;
                assert_eq!(aliquot_sum(number), expected);
            }
        )*
        }
    }

    test_aliquot_sum! {
        test_with_1: (1, 0),
        test_with_2: (2, 1),
        test_with_3: (3, 1),
        test_with_4: (4, 1+2),
        test_with_5: (5, 1),
        test_with_6: (6, 6),
        test_with_7: (7, 1),
        test_with_8: (8, 1+2+4),
        test_with_9: (9, 1+3),
        test_with_10: (10, 1+2+5),
        test_with_15: (15, 9),
        test_with_343: (343, 57),
        test_with_344: (344, 316),
        test_with_500: (500, 592),
        test_with_501: (501, 171),
    }

    #[test]
    #[should_panic]
    fn panics_if_input_is_zero() {
        aliquot_sum(0);
    }
}
