use std::ops::Add;
use std::ops::Mul;
// Rising factorials are defined as the polynomial
// (x)_n = x(x + 1)(x + 2) ... (x + n - 1).
//
// For further reading:
// https://mathworld.wolfram.com/RisingFactorial.html
// https://en.wikipedia.org/wiki/Falling_and_rising_factorials
//
// Returns the nth rising factorial. Which is calculated by giving the nth term and then
// substituting our desired x; i.e. rising_factorial(4, 5) would give 4 * 5 * 6 * 7 * 8 = 6720.
pub fn rising_factorial(x: u64, n: u64) -> u64 {
    let mut result = x;
    for i in 1..n {
        let next_term = x.add(i);
        result = result.mul(next_term);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rising_factorial() {
        assert_eq!(rising_factorial(1, 0), 1); // 0! = 1
        assert_eq!(rising_factorial(1, 1), 1); // 1 = 1
        assert_eq!(rising_factorial(2, 1), 2); // 2 = 2
        assert_eq!(rising_factorial(1, 2), 2); // 1 * 2 = 2
        assert_eq!(rising_factorial(2, 2), 6); // 2 * 3 = 6
        assert_eq!(rising_factorial(3, 3), 60); // 3 * 4 * 5 = 60
    }
}
