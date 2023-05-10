/// Iteratively sums the digits of a signed integer
///
/// ## Arguments
///
/// * `num` - The number to sum the digits of
///
/// ## Examples
///
/// ```
/// use the_algorithms_rust::math::sum_digits_iterative;
///
/// assert_eq!(10, sum_digits_iterative(1234));
/// assert_eq!(12, sum_digits_iterative(-246));
/// ```
pub fn sum_digits_iterative(num: i32) -> u32 {
    // convert to unsigned integer
    let mut num: u32 = num.unsigned_abs();
    // initialize sum
    let mut result: u32 = 0;

    // iterate through digits
    while num > 0 {
        // extract next digit and add to sum
        result += num % 10;
        num /= 10; // chop off last digit
    }
    result
}

/// Recursively sums the digits of a signed integer
///
/// ## Arguments
///
/// * `num` - The number to sum the digits of
///
/// ## Examples
///
/// ```
/// use the_algorithms_rust::math::sum_digits_recursive;
///
/// assert_eq!(10, sum_digits_recursive(1234));
/// assert_eq!(12, sum_digits_recursive(-246));
/// ```
pub fn sum_digits_recursive(num: i32) -> u32 {
    // convert to unsigned integer
    let num: u32 = num.unsigned_abs();
    // base case
    if num < 10 {
        return num;
    }
    // recursive case: add last digit to sum of remaining digits
    num % 10 + sum_digits_recursive((num / 10) as i32)
}

#[cfg(test)]
mod tests {
    mod iterative {
        // import relevant sum_digits function
        use super::super::sum_digits_iterative as sum_digits;

        #[test]
        fn zero() {
            assert_eq!(0, sum_digits(0));
        }
        #[test]
        fn positive_number() {
            assert_eq!(1, sum_digits(1));
            assert_eq!(10, sum_digits(1234));
            assert_eq!(14, sum_digits(42161));
            assert_eq!(6, sum_digits(500010));
        }
        #[test]
        fn negative_number() {
            assert_eq!(1, sum_digits(-1));
            assert_eq!(12, sum_digits(-246));
            assert_eq!(2, sum_digits(-11));
            assert_eq!(14, sum_digits(-42161));
            assert_eq!(6, sum_digits(-500010));
        }
        #[test]
        fn trailing_zeros() {
            assert_eq!(1, sum_digits(1000000000));
            assert_eq!(3, sum_digits(300));
        }
    }

    mod recursive {
        // import relevant sum_digits function
        use super::super::sum_digits_recursive as sum_digits;

        #[test]
        fn zero() {
            assert_eq!(0, sum_digits(0));
        }
        #[test]
        fn positive_number() {
            assert_eq!(1, sum_digits(1));
            assert_eq!(10, sum_digits(1234));
            assert_eq!(14, sum_digits(42161));
            assert_eq!(6, sum_digits(500010));
        }
        #[test]
        fn negative_number() {
            assert_eq!(1, sum_digits(-1));
            assert_eq!(12, sum_digits(-246));
            assert_eq!(2, sum_digits(-11));
            assert_eq!(14, sum_digits(-42161));
            assert_eq!(6, sum_digits(-500010));
        }
        #[test]
        fn trailing_zeros() {
            assert_eq!(1, sum_digits(1000000000));
            assert_eq!(3, sum_digits(300));
        }
    }
}
