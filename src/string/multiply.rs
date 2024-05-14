/*
Given two non-negative integers num1 and num2 expressed as strings, returns the product of num1 and num2, which is also expressed as a string.
Note: You cannot use any of the built-in BigInteger libraries or directly convert the input to an integer.
*/

pub fn multiply(num1: String, num2: String) -> String {
    if num1 == "0" || num2 == "0" || num1.is_empty() || num2.is_empty() {
        return "0".to_string();
    }

    let m = num1.len();
    let n = num2.len();

    let mut mult = vec![0; m + n];

    for (i, c1) in num1.chars().rev().enumerate() {
        for (j, c2) in num2.chars().rev().enumerate() {
            let n1 = c1 as u8 - b'0';
            let n2 = c2 as u8 - b'0';
            let mul = n1 * n2;
            // It could be a two-digit number here.
            mult[i + j + 1] += (mult[i + j] + mul) / 10;
            // Handling rounding. Here's a single digit.
            mult[i + j] = (mult[i + j] + mul) % 10;
        }
    }
    if mult[m + n - 1] == 0 {
        mult.pop();
    }
    mult.iter().rev().map(|&n| n.to_string()).collect()
}

#[cfg(test)]
mod tests {
    macro_rules! test_multiply {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                use super::multiply;
                let (s, t, expected) = $inputs;
                assert_eq!(multiply(s, t), expected);
            }
        )*
        }
    }

    test_multiply! {
        multiply0: ("2".to_string(), "3".to_string(), "6"),
        multiply1: ("123".to_string(), "456".to_string(), "56088"),
        multiply_zero: ("0".to_string(), "222".to_string(), "0"),
        multiply_empty: ("".to_string(), "222".to_string(), "0"),
    }
}
