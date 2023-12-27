pub fn decimal_to_hexadecimal(base_num: u64) -> String {
    let mut num = base_num;
    let mut hexadecimal_num = String::new();

    loop {
        let remainder = num % 16;
        let hex_char = if remainder < 10 {
            (remainder as u8 + b'0') as char
        } else {
            (remainder as u8 - 10 + b'A') as char
        };

        hexadecimal_num.insert(0, hex_char);
        num /= 16;
        if num == 0 {
            break;
        }
    }

    hexadecimal_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(decimal_to_hexadecimal(0), "0");
    }

    #[test]
    fn test_single_digit_decimal() {
        assert_eq!(decimal_to_hexadecimal(9), "9");
    }

    #[test]
    fn test_single_digit_hexadecimal() {
        assert_eq!(decimal_to_hexadecimal(12), "C");
    }

    #[test]
    fn test_multiple_digit_hexadecimal() {
        assert_eq!(decimal_to_hexadecimal(255), "FF");
    }

    #[test]
    fn test_big() {
        assert_eq!(decimal_to_hexadecimal(u64::MAX), "FFFFFFFFFFFFFFFF");
    }

    #[test]
    fn test_random() {
        assert_eq!(decimal_to_hexadecimal(123456), "1E240");
    }
}
