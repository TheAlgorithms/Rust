use num_bigint::BigUint;
use num_traits::One;

// A trivial example of using Big integer math

pub fn factorial(num: u32) -> BigUint {
    let mut result: BigUint = One::one();
    for i in 1..=num {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn basic_factorial() {
        assert_eq!(factorial(10), BigUint::from_str("3628800").unwrap());
        assert_eq!(
            factorial(50),
            BigUint::from_str("30414093201713378043612608166064768844377641568960512000000000000").unwrap()
        );
    }
}
