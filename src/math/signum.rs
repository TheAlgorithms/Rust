/// Signum function is a mathematical function that extracts
/// the sign of a real number. It is also known as the sign function,
/// and it is an odd piecewise function.
/// If a number is negative, i.e. it is less than zero, then sgn(x) = -1
/// If a number is zero, then sgn(0) = 0
/// If a number is positive, i.e. it is greater than zero, then sgn(x) = 1

pub fn signum(number: f64) -> i8 {
    if number == 0.0 {
        return 0;
    } else if number > 0.0 {
        return 1;
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_integer() {
        assert_eq!(signum(15.0), 1);
    }

    #[test]
    fn negative_integer() {
        assert_eq!(signum(-30.0), -1);
    }

    #[test]
    fn zero() {
        assert_eq!(signum(0.0), 0);
    }
}
