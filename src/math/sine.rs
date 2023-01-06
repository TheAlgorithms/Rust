// Calculate Sine function.
// Formula: sine(x) = x - x^3/3! + x^5/5! - x^7/7! + ...
// Where: x = angle in randians.
// It is not a real function so I will just do 9 loops, it's just an approximation.
// Source:
//     https://web.archive.org/web/20221111013039/https://www.homeschoolmath.net/teaching/sine_calculator.php

use std::f32::consts::PI;

fn factorial(num: u64) -> u64 {
    (1..=num).product()
}

pub fn sine(angle: f64) -> f64 {
    // Simplify the angle
    let angle = angle % (2.0 * PI as f64);

    let mut result = angle;
    let mut a: u64 = 3;
    let mut b = -1.0;

    for _ in 0..9 {
        result += b * (angle.powi(a as i32)) / (factorial(a) as f64);

        b = -b;
        a += 2;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{sine, PI};

    fn assert(angle: f64, expected_result: f64) {
        // I will round the result to 3 decimal places, since it's an approximation.
        assert_eq!(
            format!("{:.3}", sine(angle)),
            format!("{:.3}", expected_result)
        );
    }

    #[test]
    fn test_sine() {
        assert(0.0, 0.0);
        assert(PI as f64 / 2.0, 1.0);
        assert(PI as f64 / 4.0, 1.0 / f64::sqrt(2.0));
        assert(PI as f64, -0.0);
        assert(PI as f64 * 3.0 / 2.0, -1.0);
        assert(PI as f64 * 2.0, 0.0);
        assert(PI as f64 * 2.0 * 3.0, 0.0);
        assert(-PI as f64, 0.0);
        assert(-PI as f64 / 2.0, -1.0);
        assert(PI as f64 * 8.0 / 45.0, 0.5299192642);
        assert(0.5, 0.4794255386);
    }
}
