/// Fibonacci via Dynamic Programming

/// fibonacci(n) returns the nth fibonacci number
/// This function uses the definition of Fibonacci where:
/// F(0) = F(1) = 1 and F(n+1) = F(n) + F(n-1) for n>0
///
/// Warning: This will overflow the 128-bit unsigned integer at n=186
pub fn fibonacci(n: u32) -> u128 {
    // Use a and b to store the previous two values in the sequence
    let mut a = 0;
    let mut b = 1;
    for i in 0..n {
        // As we iterate through, move b's value into a and the new computed
        // value into b.
        let c = a+b;
        a = b;
        b = c;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::fibonacci;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 1);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 2);
        assert_eq!(fibonacci(3), 3);
        assert_eq!(fibonacci(4), 5);
        assert_eq!(fibonacci(5), 8);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(20), 10946);
        assert_eq!(fibonacci(100), 573147844013817084101);
        assert_eq!(fibonacci(184), 205697230343233228174223751303346572685);
    }
}
