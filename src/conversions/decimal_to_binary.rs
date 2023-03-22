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
    bits.rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converting_decimal_to_binary() {
        assert_eq!(decimal_to_binary(542), "1000011110");
        assert_eq!(decimal_to_binary(92), "1011100");
    }
}
