// In mathematics and computer science, the ceiling function maps x to the least integer greater than or equal to x
// Source: https://en.wikipedia.org/wiki/Floor_and_ceiling_functions

pub fn ceil(x: f64) -> f64 {
    let x_round = x.round();
    if (x_round * 10.0).round() < (x * 10.0).round() {
        x_round + 1.0
    } else {
        x_round
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_decimal() {
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
