// Author: NithinU2802
// Decimal to Octal Converter: Converts Decimal to Octal
// Wikipedia References:
// 1. https://en.wikipedia.org/wiki/Decimal
// 2. https://en.wikipedia.org/wiki/Octal

pub fn decimal_to_octal(decimal_num: u64) -> String {
    if decimal_num == 0 {
        return "0".to_string();
    }

    let mut num = decimal_num;
    let mut octal = String::new();

    while num > 0 {
        let remainder = num % 8;
        octal.push_str(&remainder.to_string());
        num /= 8;
    }

    // Reverse the string to get the correct octal representation
    octal.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_to_octal() {
        assert_eq!(decimal_to_octal(8), "10");
        assert_eq!(decimal_to_octal(15), "17");
        assert_eq!(decimal_to_octal(255), "377");
        assert_eq!(decimal_to_octal(100), "144");
        assert_eq!(decimal_to_octal(0), "0");
    }
}
