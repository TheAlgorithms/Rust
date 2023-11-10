// Author: cyrixninja
// Octal to Decimal Converter: Converts Octal to Decimal
// Wikipedia References:
// 1. https://en.wikipedia.org/wiki/Octal
// 2. https://en.wikipedia.org/wiki/Decimal

pub fn octal_to_decimal(octal_str: &str) -> Result<u64, &'static str> {
    let octal_str = octal_str.trim();

    if octal_str.is_empty() {
        return Err("Empty");
    }

    if !octal_str.chars().all(|c| ('0'..='7').contains(&c)) {
        return Err("Non-octal Value");
    }

    // Convert octal to decimal and directly return the Result
    u64::from_str_radix(octal_str, 8).map_err(|_| "Conversion error")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let input = "";
        let expected = Err("Empty");
        assert_eq!(octal_to_decimal(input), expected);
    }

    #[test]
    fn test_invalid_octal() {
        let input = "89";
        let expected = Err("Non-octal Value");
        assert_eq!(octal_to_decimal(input), expected);
    }

    #[test]
    fn test_valid_octal() {
        let input = "123";
        let expected = Ok(83);
        assert_eq!(octal_to_decimal(input), expected);
    }

    #[test]
    fn test_valid_octal2() {
        let input = "1234";
        let expected = Ok(668);
        assert_eq!(octal_to_decimal(input), expected);
    }

    #[test]
    fn test_valid_octal3() {
        let input = "12345";
        let expected = Ok(5349);
        assert_eq!(octal_to_decimal(input), expected);
    }
}
