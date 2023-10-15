/**
 * This algorithm demonstrates how to add two integers without using the + operator
 * but instead relying on bitwise operations, like bitwise XOR and AND, to simulate
 * the addition. It leverages bit manipulation to compute the sum efficiently.
 */

pub fn add_two_integers(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    let mut carry;
    let mut sum;

    // Iterate until there is no carry left
    while b != 0 {
        sum = a ^ b; // XOR operation to find the sum without carry
        carry = (a & b) << 1; // AND operation to find the carry, shifted left by 1
        a = sum;
        b = carry;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::add_two_integers;

    #[test]
    fn test_add_two_integers_positive() {
        assert_eq!(add_two_integers(3, 5), 8);
        assert_eq!(add_two_integers(100, 200), 300);
        assert_eq!(add_two_integers(65535, 1), 65536);
    }

    #[test]
    fn test_add_two_integers_negative() {
        assert_eq!(add_two_integers(-10, 6), -4);
        assert_eq!(add_two_integers(-50, -30), -80);
        assert_eq!(add_two_integers(-1, -1), -2);
    }

    #[test]
    fn test_add_two_integers_zero() {
        assert_eq!(add_two_integers(0, 0), 0);
        assert_eq!(add_two_integers(0, 42), 42);
        assert_eq!(add_two_integers(0, -42), -42);
    }
}
