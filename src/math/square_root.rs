/// squre_root returns the square root
/// of a f64 number using Newtons method
pub fn square_root(num: f64) -> f64 {
    if num < 0.0_f64 {
        return f64::NAN;
    }

    let mut root = 1.0_f64;

    while (root * root - num).abs() > 1e-10_f64 {
        root -= (root * root - num) / (2.0_f64 * root);
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!((square_root(4.0_f64) - 2.0_f64).abs() <= 1e-10_f64);
        assert!(square_root(-4.0_f64).is_nan());
    }
}
