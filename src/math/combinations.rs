// Function to calculate combinations of k elements from a set of n elements
pub fn combinations(n: i64, k: i64) -> i64 {
    // Check if either n or k is negative, and panic if so
    if n < 0 || k < 0 {
        panic!("Please insert positive values");
    }

    let mut res: i64 = 1;
    for i in 0..k {
        // Calculate the product of (n - i) and update the result
        res *= n - i;
        // Divide by (i + 1) to calculate the combination
        res /= i + 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test case for combinations(10, 5)
    #[test]
    fn test_combinations_10_choose_5() {
        assert_eq!(combinations(10, 5), 252);
    }

    // Test case for combinations(6, 3)
    #[test]
    fn test_combinations_6_choose_3() {
        assert_eq!(combinations(6, 3), 20);
    }

    // Test case for combinations(20, 5)
    #[test]
    fn test_combinations_20_choose_5() {
        assert_eq!(combinations(20, 5), 15504);
    }

    // Test case for invalid input (negative values)
    #[test]
    #[should_panic(expected = "Please insert positive values")]
    fn test_combinations_invalid_input() {
        combinations(-5, 10);
    }
}
