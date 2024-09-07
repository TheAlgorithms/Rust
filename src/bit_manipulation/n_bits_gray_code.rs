/// Custom error type for Gray code generation.
#[derive(Debug, PartialEq)]
pub enum GrayCodeError {
    ZeroBitCount,
}

/// Generates an n-bit Gray code sequence using the direct Gray code formula.
///
/// # Arguments
///
/// * `n` - The number of bits for the Gray code.
///
/// # Returns
///
/// A vector of Gray code sequences as strings.
pub fn generate_gray_code(n: usize) -> Result<Vec<String>, GrayCodeError> {
    if n == 0 {
        return Err(GrayCodeError::ZeroBitCount);
    }

    let num_codes = 1 << n;
    let mut result = Vec::with_capacity(num_codes);

    for i in 0..num_codes {
        let gray = i ^ (i >> 1);
        let gray_code = (0..n)
            .rev()
            .map(|bit| if gray & (1 << bit) != 0 { '1' } else { '0' })
            .collect::<String>();
        result.push(gray_code);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! gray_code_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $test_case;
                    assert_eq!(generate_gray_code(input), expected);
                }
            )*
        };
    }

    gray_code_tests! {
        zero_bit_count: (0, Err(GrayCodeError::ZeroBitCount)),
        gray_code_1_bit: (1, Ok(vec![
            "0".to_string(),
            "1".to_string(),
        ])),
        gray_code_2_bit: (2, Ok(vec![
            "00".to_string(),
            "01".to_string(),
            "11".to_string(),
            "10".to_string(),
        ])),
        gray_code_3_bit: (3, Ok(vec![
            "000".to_string(),
            "001".to_string(),
            "011".to_string(),
            "010".to_string(),
            "110".to_string(),
            "111".to_string(),
            "101".to_string(),
            "100".to_string(),
        ])),
    }
}
