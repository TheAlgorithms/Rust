/// ## Euler's Totient (`φ(n)`)
///
/// Calculate the **Euler's Totient** function of a given number `n`.
///
/// Wikipedia: https://en.wikipedia.org/wiki/Euler%27s_totient_function
pub fn euler_totient(mut n: i64) -> i64 {
    let mut result = n;

    if n <= 0 {
        panic!("Must be a positive integer!");
    }

    for i in 2..=n.isqrt() {
        if n % i == 0 {
            while n % i == 0 {
                n /= i;
            }
            result -= result / i;
        }
    }

    if n > 1 {
        result -= result / n;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler_totient() {
        assert_eq!(euler_totient(1), 1);
        assert_eq!(euler_totient(325), 240);
        assert_eq!(euler_totient(746), 372);
        assert_eq!(euler_totient(3_639), 2_424);
        assert_eq!(euler_totient(98_354), 49_176);
        assert_eq!(euler_totient(123_456), 41_088);
        assert_eq!(euler_totient(493_123_235), 347_518_080);
        assert_eq!(euler_totient(945_243_784_032), 315_074_904_192);
        assert_eq!(euler_totient(372_036_854_775_808), 185_661_377_740_800);
    }
}
