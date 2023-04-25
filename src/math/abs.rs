/// This function returns the absolute value of a number.\
/// The absolute value of a number is the non-negative value of the number, regardless of its sign.\
///
/// Wikipedia: <https://en.wikipedia.org/wiki/Absolute_value>
pub fn abs(num: f64) -> f64 {
    if num < 0.0 {
        return -num;
    }

    num
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn negative_number() {
        assert_eq!(420.0, abs(-420.0));
    }

    #[test]
    fn zero() {
        assert_eq!(0.0, abs(0.0));
    }

    #[test]
    fn positive_number() {
        assert_eq!(69.69, abs(69.69));
    }
}
