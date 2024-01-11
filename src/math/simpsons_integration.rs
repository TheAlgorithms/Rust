pub fn simpsons_integration<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (b - a) / n as f64;
    (0..n)
        .map(|i| {
            let x0 = a + i as f64 * h;
            let x1 = x0 + h / 2.0;
            let x2 = x0 + h;
            (h / 6.0) * (f(x0) + 4.0 * f(x1) + f(x2))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simpsons_integration() {
        let f = |x: f64| x.powi(2);
        let a = 0.0;
        let b = 1.0;
        let n = 100;
        let result = simpsons_integration(f, a, b, n);
        assert!((result - 1.0 / 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_error() {
        let f = |x: f64| x.powi(2);
        let a = 0.0;
        let b = 1.0;
        let n = 100;
        let result = simpsons_integration(f, a, b, n);
        let error = (1.0 / 3.0 - result).abs();
        assert!(error < 1e-6);
    }

    #[test]
    fn test_convergence() {
        let f = |x: f64| x.powi(2);
        let a = 0.0;
        let b = 1.0;
        let n = 100;
        let result1 = simpsons_integration(f, a, b, n);
        let result2 = simpsons_integration(f, a, b, 2 * n);
        let result3 = simpsons_integration(f, a, b, 4 * n);
        let result4 = simpsons_integration(f, a, b, 8 * n);
        assert!((result1 - result2).abs() < 1e-6);
        assert!((result2 - result3).abs() < 1e-6);
        assert!((result3 - result4).abs() < 1e-6);
    }

    #[test]
    fn test_negative() {
        let f = |x: f64| -x.powi(2);
        let a = 0.0;
        let b = 1.0;
        let n = 100;
        let result = simpsons_integration(f, a, b, n);
        assert!((result + 1.0 / 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_non_zero_lower_bound() {
        let f = |x: f64| x.powi(2);
        let a = 1.0;
        let b = 2.0;
        let n = 100;
        let result = simpsons_integration(f, a, b, n);
        assert!((result - 7.0 / 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_non_zero_upper_bound() {
        let f = |x: f64| x.powi(2);
        let a = 0.0;
        let b = 2.0;
        let n = 100;
        let result = simpsons_integration(f, a, b, n);
        assert!((result - 8.0 / 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_non_zero_lower_and_upper_bound() {
        let f = |x: f64| x.powi(2);
        let a = 1.0;
        let b = 2.0;
        let n = 100;
        let result = simpsons_integration(f, a, b, n);
        assert!((result - 7.0 / 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_non_zero_lower_and_upper_bound_negative() {
        let f = |x: f64| -x.powi(2);
        let a = 1.0;
        let b = 2.0;
        let n = 100;
        let result = simpsons_integration(f, a, b, n);
        assert!((result + 7.0 / 3.0).abs() < 1e-6);
    }

    #[test]
    fn parabola_curve_length() {
        // Calculate the length of the curve f(x) = x^2 for -5 <= x <= 5
        // We should integrate sqrt(1 + (f'(x))^2)
        let function = |x: f64| -> f64 { (1.0 + 4.0 * x * x).sqrt() };
        let result = simpsons_integration(function, -5.0, 5.0, 1_000);
        let integrated = |x: f64| -> f64 { (x * function(x) / 2.0) + ((2.0 * x).asinh() / 4.0) };
        let expected = integrated(5.0) - integrated(-5.0);
        assert!((result - expected).abs() < 1e-9);
    }

    #[test]
    fn area_under_cosine() {
        use std::f64::consts::PI;
        // Calculate area under f(x) = cos(x) + 5 for -pi <= x <= pi
        // cosine should cancel out and the answer should be 2pi * 5
        let function = |x: f64| -> f64 { x.cos() + 5.0 };
        let result = simpsons_integration(function, -PI, PI, 1_000);
        let expected = 2.0 * PI * 5.0;
        assert!((result - expected).abs() < 1e-9);
    }
}
