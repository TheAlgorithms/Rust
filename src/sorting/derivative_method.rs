const DERIVATIVE_PRECISION: f64 = 0.0001;

pub fn derivative_method<F>(x: f64, y: f64, f: F) -> f64
where
    F: Fn(f64, f64) -> f64,
{
    let h = DERIVATIVE_PRECISION;
    (f(x + h, y) - f(x, y)) / h
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_function(x: f64, y: f64) -> f64 {
        x.powi(2) + y.powi(2)
    }
    
    #[test]
    fn test_derivative() {
        let x = 1.0;
        let y = 2.0;
        let f = test_function;
        let df_dx = derivative_method(x, y, f);
        let df_dy = derivative_method(y, x, f);
        assert_eq!(df_dx, 2.000100000003613);
        assert_eq!(df_dy, 4.0001000000078335);
    }

    #[test]
    fn test_error() {
        let x = 1.0;
        let y = 2.0;
        let f = test_function;
        let df_dx = derivative_method(x, y, f);
        let df_dy = derivative_method(y, x, f);
        assert_ne!(df_dx, 2.0);
        assert_ne!(df_dy, 4.0);
    }

    #[test]
    fn test_nan() {
        let x = 1.0;
        let y = 2.0;
        let f = test_function;
        let df_dx = derivative_method(x, y, f);
        let df_dy = derivative_method(y, x, f);
        assert!(!df_dx.is_nan());
        assert!(!df_dy.is_nan());
    }

    #[test]
    fn test_inf() {
        let x = 1.0;
        let y = 2.0;
        let f = test_function;
        let df_dx = derivative_method(x, y, f);
        let df_dy = derivative_method(y, x, f);
        assert!(!df_dx.is_infinite());
        assert!(!df_dy.is_infinite());
    }

    #[test]
    fn test_zero() {
        let x = 1.0;
        let y = 2.0;
        let f = test_function;
        let df_dx = derivative_method(x, y, f);
        let df_dy = derivative_method(y, x, f);
        assert_ne!(df_dx, 0.0);
        assert_ne!(df_dy, 0.0);
    }

    #[test]
    fn test_subnormal() {
        let x = 1.0;
        let y = 2.0;
        let f = test_function;
        let df_dx = derivative_method(x, y, f);
        let df_dy = derivative_method(y, x, f);
        assert!(!df_dx.is_subnormal());
        assert!(!df_dy.is_subnormal());
    }
}
