pub fn trapezoidal_integral<F>(a: f64, b: f64, f: F, precision: u32) -> f64
where
    F: Fn(f64) -> f64,
{
    let delta = (b - a) / precision as f64;

    let integral: f64 = (0..precision)
        .map(|trapezoid| {
            let left_side = a + (delta * trapezoid as f64);
            let right_side = left_side + delta;

            0.5 * (f(left_side) + f(right_side)) * delta
        })
        .sum();

    if a > b {
        -integral
    } else {
        integral
    }
}

#[allow(dead_code)]
fn main() {
    let f = |x: f64| x.powi(3);
    let result = trapezoidal_integral(0.0, 1.0, f, 1000);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integral() {
        let f = |x: f64| x.powi(2);
        let result = trapezoidal_integral(0.0, 1.0, f, 1000);
        assert!((result - 1.0 / 3.0).abs() < 0.0001);
    }

    #[test]
    fn test_precision() {
        let f = |x: f64| x.powi(2);
        let result = trapezoidal_integral(0.0, 1.0, f, 10000);
        assert!((result - 1.0 / 3.0).abs() < 0.00001);
    }

    #[test]
    fn test_negative() {
        let f = |x: f64| x.powi(2);
        let result = trapezoidal_integral(-1.0, 1.0, f, 10000);
        assert!((result - 2.0 / 3.0).abs() < 0.00001);
    }

    #[test]
    fn test_negative_precision() {
        let f = |x: f64| x.powi(2);
        let result = trapezoidal_integral(-1.0, 1.0, f, 100000);
        assert!((result - 2.0 / 3.0).abs() < 0.000001);
    }
}
