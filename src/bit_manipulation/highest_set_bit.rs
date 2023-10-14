// Find Highest Set Bit in Rust
// This code provides a function to calculate the position (or index) of the most significant bit set to 1 in a given integer.

// Define a function to find the highest set bit.
pub fn find_highest_set_bit(num: i32) -> Option<i32> {
    if num < 0 {
        // Input cannot be negative.
        panic!("Input cannot be negative");
    }

    if num == 0 {
        return None; // No bit is set, return None.
    }

    let mut position = 0;
    let mut n = num;

    while n > 0 {
        n >>= 1;
        position += 1;
    }

    Some(position - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_number() {
        let num = 18;
        assert_eq!(find_highest_set_bit(num), Some(4));
    }

    #[test]
    fn test_zero() {
        let num = 0;
        assert_eq!(find_highest_set_bit(num), None);
    }

    #[test]
    #[should_panic(expected = "Input cannot be negative")]
    fn test_negative_number() {
        let num = -12;
        find_highest_set_bit(num);
    }
}
