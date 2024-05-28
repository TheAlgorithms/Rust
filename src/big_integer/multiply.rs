/// Performs long multiplication on string representations of non-negative numbers.
pub fn multiply(num1: &str, num2: &str) -> String {
    if !is_valid_nonnegative(num1) || !is_valid_nonnegative(num2) {
        panic!("String does not conform to specification")
    }

    if num1 == "0" || num2 == "0" {
        return "0".to_string();
    }
    let output_size = num1.len() + num2.len();

    let mut mult = vec![0; output_size];
    for (i, c1) in num1.chars().rev().enumerate() {
        for (j, c2) in num2.chars().rev().enumerate() {
            let mul = c1.to_digit(10).unwrap() * c2.to_digit(10).unwrap();
            // It could be a two-digit number here.
            mult[i + j + 1] += (mult[i + j] + mul) / 10;
            // Handling rounding. Here's a single digit.
            mult[i + j] = (mult[i + j] + mul) % 10;
        }
    }
    if mult[output_size - 1] == 0 {
        mult.pop();
    }
    mult.iter().rev().map(|&n| n.to_string()).collect()
}

pub fn is_valid_nonnegative(num: &str) -> bool {
    num.chars().all(char::is_numeric) && !num.is_empty() && (!num.starts_with('0') || num == "0")
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
                assert_eq!(multiply(s, t), expected);
                assert_eq!(multiply(t, s), expected);
            }
        )*
        }
    }

    test_multiply! {
        multiply0: ("2", "3", "6"),
        multiply1: ("123", "456", "56088"),
        multiply_zero: ("0", "222", "0"),
        other_1: ("99", "99", "9801"),
        other_2: ("999", "99", "98901"),
        other_3: ("9999", "99", "989901"),
        other_4: ("192939", "9499596", "1832842552644"),
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
