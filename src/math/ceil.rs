// In mathematics and computer science, the ceiling function maps x to the least integer greater than or equal to x
// Source: https://en.wikipedia.org/wiki/Floor_and_ceiling_functions

pub fn ceil(x: f64) -> f64 {
    let x_rounded_towards_zero = x as i32 as f64;
    if x < 0. || x_rounded_towards_zero == x {
        x_rounded_towards_zero
    } else {
        x_rounded_towards_zero + 1_f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_decimal() {
        let num = 1.10;
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn positive_decimal_with_small_number() {
        let num = 3.01;
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn positive_integer() {
        let num = 1.00;
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn negative_decimal() {
        let num = -1.10;
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn negative_decimal_with_small_number() {
        let num = -1.01;
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn negative_integer() {
        let num = -1.00;
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn zero() {
        let num = 0.00;
        assert_eq!(ceil(num), num.ceil());
    }
}
