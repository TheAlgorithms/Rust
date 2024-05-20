/// Performs long multiplication on string representations of non-negative numbers.

pub fn multiply(num1: &str, num2: &str) -> String {
    if num1.is_empty() || num2.is_empty() {
        panic!("String numbers cannot be empty")
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
    #[test]
    #[should_panic]
    fn panic_when_inputs_is_empty() {
        multiply("", "121");
        multiply("", "");
    }
}
