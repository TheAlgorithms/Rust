pub fn decimal_to_binary(base_num: u64) -> String {
    let mut num = base_num;
    let mut binary_num = String::new();
    loop {
        let bit = (num % 2).to_string();
        binary_num.push_str(&bit);
        num /= 2;
        if num == 0 {
            break;
        }
    }

    let bits = binary_num.chars();
    let result = bits.rev().collect();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converting_decimal_to_binary() {
        assert_eq!(decimal_to_binary(69030), "10000110110100110");
        assert_eq!(decimal_to_binary(92), "1011100");
    }
}
