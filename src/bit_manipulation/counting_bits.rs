/*
The counting bits algorithm, also known as the "population count" or "Hamming weight,"
calculates the number of set bits (1s) in the binary representation of an unsigned integer.
It uses a technique known as Brian Kernighan's algorithm, which efficiently clears the least
significant set bit in each iteration.
*/

pub fn count_set_bits(mut n: u32) -> u32 {
    // Initialize a variable to keep track of the count of set bits
    let mut count = 0;
    while n > 0 {
        // Clear the least significant set bit by
        // performing a bitwise AND operation with (n - 1)
        n &= n - 1;

        // Increment the count for each set bit found
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_set_bits_zero() {
        assert_eq!(count_set_bits(0), 0);
    }

    #[test]
    fn test_count_set_bits_one() {
        assert_eq!(count_set_bits(1), 1);
    }

    #[test]
    fn test_count_set_bits_power_of_two() {
        assert_eq!(count_set_bits(16), 1); // 16 is 2^4, only one set bit
    }

    #[test]
    fn test_count_set_bits_all_set_bits() {
        assert_eq!(count_set_bits(u32::MAX), 32); // Maximum value for u32, all set bits
    }
}
