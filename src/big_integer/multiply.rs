/// Performs long multiplication on string representations of non-negative base-10 integers.
///
/// # Panics
/// Panics if either input contains non-ASCII digits, leading zeros (except "0"),
/// or is empty.
///
/// # Examples
/// ```
/// assert_eq!(multiply("123", "456"), "56088");
/// use crate::big_integer::multiply;
/// assert_eq!(multiply("99", "99"), "9801");
/// ```
pub fn multiply(num1: &str, num2: &str) -> String {
    if !is_valid_nonnegative(num1) {
        panic!("Invalid input: `{num1}` is not a valid non-negative integer string");
    }
    if !is_valid_nonnegative(num2) {
        panic!("Invalid input: `{num2}` is not a valid non-negative integer string");
    }

    if num1 == "0" || num2 == "0" {
        return "0".to_string();
    }

    let output_size = num1.len() + num2.len();
    let mut mult = vec![0; output_size];

    for (i, c1) in num1.chars().rev().enumerate() {
        for (j, c2) in num2.chars().rev().enumerate() {
            let mul = c1.to_digit(10).unwrap() * c2.to_digit(10).unwrap();
            let sum = mult[i + j] + mul;

            mult[i + j + 1] += sum / 10;
            mult[i + j] = sum % 10;
        }
    }

    if mult[output_size - 1] == 0 {
        mult.pop();
    }
    mult.iter().rev().map(|&n| n.to_string()).collect()
}

/// Checks whether a string represents a valid non-negative base-10 integer.
/// Disallows empty strings, leading zeros (except "0"), and non-ASCII digits.
pub fn is_valid_nonnegative(num: &str) -> bool {
    !num.is_empty()
        && num.chars().all(|c| c.is_ascii_digit())
        && (!num.starts_with('0') || num == "0")
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_multiply {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (s, t, expected) = $inputs;
                    assert_eq!(multiply(s, t), expected, "multiply({s}, {t})");
                    assert_eq!(multiply(t, s), expected, "multiply({t}, {s})");
                }
            )*
        }
    }

    test_multiply! {
        multiply_small: ("2", "3", "6"),
        multiply_basic: ("123", "456", "56088"),
        multiply_zero: ("0", "222", "0"),
        multiply_double_digits: ("99", "99", "9801"),
        multiply_triple_digits: ("999", "99", "98901"),
        multiply_four_digits: ("9999", "99", "989901"),
        multiply_large: ("192939", "9499596", "1832842552644"),
    }

    macro_rules! test_multiply_with_wrong_input {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                #[should_panic]
                fn $name() {
                    let (s, t) = $inputs;
                    multiply(s, t);
                }
            )*
        }
    }

    test_multiply_with_wrong_input! {
        empty_input: ("", "121"),
        leading_zero: ("01", "3"),
        wrong_characters: ("2", "12d4"),
        wrong_input_and_zero_1: ("0", "x"),
        wrong_input_and_zero_2: ("y", "0"),
    }
}
