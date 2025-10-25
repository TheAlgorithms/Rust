/// A palindrome number is a number that reads the same backward as forward.
/// For example, 121 is a palindrome, but 123 is not.
/// This function checks if a given unsigned 64-bit integer 'number' is a palindrome
/// by mathematically reversing its digits and comparing it to the original.
/// Note: By this definition, negative numbers are not considered palindromes.

pub fn is_palindrome(number: u64) -> bool {
    // A single-digit number is always a palindrome
    if number < 10 {
        return true;
    }

    let original_number = number;
    let mut reversed_number: u64 = 0;
    let mut n = number;

    // Loop until all digits of n have been processed
    while n > 0 {
        // Get the last digit
        let remainder = n % 10;
        
        // Build the reversed number
        reversed_number = (reversed_number * 10) + remainder;
        
        // Remove the last digit
        n /= 10;
    }

    // Check if the original number equals its reversed version
    original_number == reversed_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_palindrome() {
        assert!(is_palindrome(121));
    }

    #[test]
    fn standard_non_palindrome() {
        assert!(!is_palindrome(123));
    }

    #[test]
    fn single_digit() {
        // Single digits are always palindromes
        assert!(is_palindrome(7));
    }

    #[test]
    fn zero() {
        // Zero is a palindrome
        assert!(is_palindrome(0));
    }

    #[test]
    fn large_palindrome() {
        assert!(is_palindrome(123454321));
    }

    #[test]
    fn number_ending_in_zero() {
        // No number > 0 that ends in 0 can be a palindrome
        assert!(!is_palindrome(120));
    }
}
