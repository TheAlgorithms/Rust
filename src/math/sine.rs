/// Sine function for non radian angle
///
/// Interprets the argument in degrees, not in radians
///
/// ### Example
///
/// sin(1<sup>o</sup>) != \[ sin(1 rad) == sin(π/180) \]
pub fn sine_no_radian_arg<T: Into<f64>>(x: T, tol: f64) -> f64 {
    use std::f64::consts::PI;
    let val: f64 = x.into();
    sine(val * PI / 180f64, tol)
}

/// Returns the value of sin(x), approximated with the given tolerance
///
/// This function supposes the argument is in radians
///
/// ### Example
///
/// sin(1) == sin(1 rad) == sin(π/180)
pub fn sine<T: Into<f64>>(x: T, tol: f64) -> f64 {
    use std::f64::consts::PI;
    const PERIOD: f64 = 2.0 * PI;
    /* Sometimes, this function is called for a big 'n'(when tol is very small) */
    fn factorial(n: i128) -> i128 {
        (1..=n).product()
    }

    fn round_up_to_decimal(x: f64, decimal: i32) -> f64 {
        let multiplier = 10f64.powi(decimal);
        (x * multiplier).round() / multiplier
    }

    let mut value: f64 = x.into(); //<-- This is the line for which the trait 'Into' is required
    
    /* Check for invalid arguments */
    if !value.is_finite() || value.is_nan() {
        println!("sine does not accept invalid arguments.");
        return f64::NAN;
    }

    /*
        The argument to sine could be bigger than the sine's PERIOD
        To prevent overflowing, strip the value off relative to the PERIOD
    */
    while value >= PERIOD {
        value -= PERIOD;
    }
    /* For cases when the value is smaller than the -PERIOD (e.g. sin(-3π) <=> sin(-π)) */
    while value <= -PERIOD {
        value += PERIOD;
    }

    let mut rez = 0f64;
    let mut prev_rez = 1f64;
    let mut step: i32 = 0;
    /*
        This function uses the MacLaurin Series for sin(x)
        sin(x) = Σ (-1)^n * x^2n+1 / (2n+1)!, for n >= 0 and x a Real number
    */
    while (prev_rez - rez).abs() > tol {
        prev_rez = rez;
        rez += (-1f64).powi(step) * value.powi(2 * step + 1)
            / factorial((2 * step + 1) as i128) as f64;
        step += 1;
    }

    /* Round up to the 5th decimal */
    round_up_to_decimal(rez, 5)
}

#[cfg(test)]
mod tests {
    use super::{sine, sine_no_radian_arg};
    use std::f64::consts::PI;

    fn assert(angle: f64, expected_result: f64, is_radian: bool) {
        // I will round the result to 3 decimal places, since it's an approximation.
        match is_radian {
            true => assert_eq!(
                format!("{:.3}", sine(angle, 1e-10)),
                /* Lower the tolerance, the more accurate the value will be */
                format!("{:.3}", expected_result)
            ),
            false => assert_eq!(
                format!("{:.3}", sine_no_radian_arg(angle, 1e-10)),
                format!("{:.3}", expected_result)
            )
        }
    }

    #[test]
    fn test_sine() {
        assert(0.0, 0.0, true);
        assert(PI / 2.0, 1.0, true);
        assert(PI / 4.0, 1.0 / f64::sqrt(2.0), true);
        assert(PI, -0.0, true);
        assert(PI * 3.0 / 2.0, -1.0, true);
        assert(PI * 2.0, 0.0, true);
        assert(PI * 2.0 * 3.0, 0.0, true);
        assert(-PI, 0.0, true);
        assert(-PI / 2.0, -1.0, true);
        assert(PI * 8.0 / 45.0, 0.5299192642, true);
        assert(0.5, 0.4794255386, true);
        assert(0.0, 0.0, false);
        assert(90f64, 1.0, false);
        assert(45f64, 1.0 / f64::sqrt(2.0), false);
        assert(180f64, -0.0, false);
        assert(180f64 * 3.0 / 2.0, -1.0, false);
        assert(180f64 * 2.0, 0.0, false);
        assert(180f64 * 2.0 * 3.0, 0.0, false);
        assert(-180f64, 0.0, false);
        assert(-180f64 / 2.0, -1.0, false);
        assert(180f64 * 8.0 / 45.0, 0.5299192642, false);
        assert(0.5, 0.00872654, false);

    }
}
