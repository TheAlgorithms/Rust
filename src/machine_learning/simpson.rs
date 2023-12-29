pub fn simpsons_rule<F>(f: F, a: f64, b: f64, n: usize) -> f64
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
    fn test_simpsons_rule() {
        let f = |x: f64| x.powi(2);
        let a = 0.0;
        let b = 1.0;
        let n = 100;
        let result = simpsons_rule(f, a, b, n);
        assert!((result - 1.0 / 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_error() {
        let f = |x: f64| x.powi(2);
        let a = 0.0;
        let b = 1.0;
        let n = 100;
        let result = simpsons_rule(f, a, b, n);
        let error = (1.0 / 3.0 - result).abs();
        assert!(error < 1e-6);
    }

    #[test]
    fn test_convergence() {
        let f = |x: f64| x.powi(2);
        let a = 0.0;
        let b = 1.0;
        let n = 100;
        let result1 = simpsons_rule(f, a, b, n);
        let result2 = simpsons_rule(f, a, b, 2 * n);
        let result3 = simpsons_rule(f, a, b, 4 * n);
        let result4 = simpsons_rule(f, a, b, 8 * n);
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
        let result = simpsons_rule(f, a, b, n);
        assert!((result + 1.0 / 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_non_zero_lower_bound() {
        let f = |x: f64| x.powi(2);
        let a = 1.0;
        let b = 2.0;
        let n = 100;
        let result = simpsons_rule(f, a, b, n);
        assert!((result - 7.0 / 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_non_zero_upper_bound() {
        let f = |x: f64| x.powi(2);
        let a = 0.0;
        let b = 2.0;
        let n = 100;
        let result = simpsons_rule(f, a, b, n);
        assert!((result - 8.0 / 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_non_zero_lower_and_upper_bound() {
        let f = |x: f64| x.powi(2);
        let a = 1.0;
        let b = 2.0;
        let n = 100;
        let result = simpsons_rule(f, a, b, n);
        assert!((result - 7.0 / 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_non_zero_lower_and_upper_bound_negative() {
        let f = |x: f64| -x.powi(2);
        let a = 1.0;
        let b = 2.0;
        let n = 100;
        let result = simpsons_rule(f, a, b, n);
        assert!((result + 7.0 / 3.0).abs() < 1e-6);
    }
}
