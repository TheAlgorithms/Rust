pub fn is_armstrong_number(number: u32) -> bool {
    let mut digits: Vec<u32> = Vec::new();
    let mut num: u32 = number;

    loop {
        digits.push(num % 10);
        num /= 10;
        if num == 0 {
            break;
        }
    }

    let sum_nth_power_of_digits: u32 = digits
        .iter()
        .map(|digit| digit.pow(digits.len() as u32))
        .sum();
    sum_nth_power_of_digits == number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_digit_armstrong_number() {
        assert!(is_armstrong_number(1))
    }
    #[test]
    fn two_digit_numbers_are_not_armstrong_numbers() {
        assert!(!is_armstrong_number(15))
    }
    #[test]
    fn three_digit_armstrong_number() {
        assert!(is_armstrong_number(153))
    }
    #[test]
    fn three_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(105))
    }
    #[test]
    fn big_armstrong_number() {
        assert!(is_armstrong_number(912985153))
    }
}
