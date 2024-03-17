/// squre_root returns the square root
/// of a f64 number using Newton's method
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

// fast_inv_sqrt returns an approximation of the inverse square root
// This algorithm was first used in Quake and has been reimplemented in a few other languages
// This crate implements it more thoroughly: https://docs.rs/quake-inverse-sqrt/latest/quake_inverse_sqrt/
pub fn fast_inv_sqrt(num: f32) -> f32 {
    // If you are confident in your input this can be removed for speed
    if num < 0.0f32 {
        return f32::NAN;
    }

    let i = num.to_bits();
    let i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);

    println!("num: {:?}, out: {:?}", num, y * (1.5 - 0.5 * num * y * y));
    // First iteration of Newton's approximation
    y * (1.5 - 0.5 * num * y * y)
    // The above can be repeated for more precision
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_inv_sqrt() {
        // Negatives don't have square roots:
        assert!(fast_inv_sqrt(-1.0f32).is_nan());

        // Test a few cases, expect less than 1% error:
        let test_pairs = [(4.0, 0.5), (16.0, 0.25), (25.0, 0.2)];
        for pair in test_pairs {
            assert!((fast_inv_sqrt(pair.0) - pair.1).abs() <= (0.01 * pair.0));
        }
    }

    #[test]
    fn test_sqare_root() {
        assert!((square_root(4.0_f64) - 2.0_f64).abs() <= 1e-10_f64);
        assert!(square_root(-4.0_f64).is_nan());
    }
}
