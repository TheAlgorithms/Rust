//! Bisection method for finding a root of a continuous function.
//!
//! Given an interval `[lower, upper]` where `f(lower)` and `f(upper)` have
//! opposite signs, a root is guaranteed to exist inside it (intermediate value
//! theorem). Each step halves the interval, keeping the half that still has a
//! sign change.
//!
//! Unlike Newton-Raphson this needs no derivative and cannot diverge, but it
//! converges more slowly (one bit of accuracy per iteration).

/// Finds a root of `f` inside `[lower, upper]` using the bisection method.
///
/// Returns `None` if:
/// - a bound is NaN or infinite, or `tolerance` is not a positive number,
/// - `f` is NaN at either bound,
/// - `f(lower)` and `f(upper)` have the same sign (no root is guaranteed), or
/// - the interval does not shrink below `tolerance` within `max_iterations`.
pub fn bisection<F>(
    f: F,
    lower: f64,
    upper: f64,
    tolerance: f64,
    max_iterations: usize,
) -> Option<f64>
where
    F: Fn(f64) -> f64,
{
    if !lower.is_finite() || !upper.is_finite() || !tolerance.is_finite() || tolerance <= 0.0 {
        return None;
    }

    let (mut lower, mut upper) = if lower <= upper {
        (lower, upper)
    } else {
        (upper, lower)
    };

    let f_lower = f(lower);
    let f_upper = f(upper);

    if f_lower.is_nan() || f_upper.is_nan() {
        return None;
    }

    if f_lower == 0.0 {
        return Some(lower);
    }

    if f_upper == 0.0 {
        return Some(upper);
    }

    if f_lower.signum() == f_upper.signum() {
        return None;
    }

    let f_lower_sign = f_lower.signum();

    for _ in 0..max_iterations {
        let mid = f64::midpoint(lower, upper);

        if upper - lower <= tolerance {
            return Some(mid);
        }

        let f_mid = f(mid);
        if f_mid == 0.0 {
            return Some(mid);
        }

        // move lower to mid if only they have the samve sign bit.
        // As long as we have this if check we don't need to update
        // the `f_lower_sign` because it's sign bit will be preserved.
        if f_mid.signum() == f_lower_sign {
            lower = mid;
        } else {
            upper = mid;
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    fn assert_approx_eq(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn finds_root_of_linear_function() {
        let root = bisection(|x| 2.0 * x - 6.0, 0.0, 5.0, EPSILON, 100);

        assert_approx_eq(root.unwrap(), 3.0, EPSILON);
    }

    #[test]
    fn finds_root_of_quadratic_function() {
        let root = bisection(|x| x * x - 4.0, 0.0, 3.0, EPSILON, 100);

        assert_approx_eq(root.unwrap(), 2.0, EPSILON);
    }

    #[test]
    fn finds_root_of_nonlinear_function() {
        let root = bisection(|x| x.cos() - x, 0.0, 1.0, EPSILON, 100);

        assert_approx_eq(root.unwrap(), 0.739085, EPSILON);
    }

    #[test]
    fn finds_root_when_lower_bound_is_root() {
        let root = bisection(|x| x, 0.0, 5.0, EPSILON, 100);

        assert_eq!(root, Some(0.0));
    }

    #[test]
    fn finds_root_when_upper_bound_is_root() {
        let root = bisection(|x| x, -5.0, 0.0, EPSILON, 100);

        assert_eq!(root, Some(0.0));
    }

    #[test]
    fn handles_reversed_bounds() {
        let root = bisection(|x| x - 2.0, 5.0, 0.0, EPSILON, 100);

        assert_approx_eq(root.unwrap(), 2.0, EPSILON);
    }

    #[test]
    fn accepts_root_at_negative_zero() {
        let root = bisection(|x| x + 1.0, -1.0, 5.0, EPSILON, 100);

        assert_eq!(root, Some(-1.0));
    }

    #[test]
    fn returns_none_when_bounds_have_same_sign() {
        let root = bisection(|x| x * x + 1.0, -2.0, 2.0, EPSILON, 100);

        assert_eq!(root, None);
    }

    #[test]
    fn returns_none_when_function_is_nan_at_lower_bound() {
        let root = bisection(
            |x| if x == 0.0 { f64::NAN } else { x - 1.0 },
            0.0,
            2.0,
            EPSILON,
            100,
        );

        assert_eq!(root, None);
    }

    #[test]
    fn returns_none_when_function_is_nan_at_upper_bound() {
        let root = bisection(
            |x| if x == 2.0 { f64::NAN } else { x - 1.0 },
            0.0,
            2.0,
            EPSILON,
            100,
        );

        assert_eq!(root, None);
    }

    #[test]
    fn returns_none_for_nan_bounds() {
        assert_eq!(bisection(|x| x, f64::NAN, 1.0, EPSILON, 100), None);

        assert_eq!(bisection(|x| x, 0.0, f64::NAN, EPSILON, 100), None);
    }

    #[test]
    fn returns_none_for_infinite_bounds() {
        assert_eq!(bisection(|x| x, f64::NEG_INFINITY, 1.0, EPSILON, 100), None);

        assert_eq!(bisection(|x| x, 0.0, f64::INFINITY, EPSILON, 100), None);
    }

    #[test]
    fn returns_none_for_invalid_tolerance() {
        for tolerance in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(
                bisection(|x| x - 1.0, 0.0, 2.0, tolerance, 100),
                None,
                "unexpected result for tolerance = {tolerance}"
            );
        }
    }

    #[test]
    fn returns_none_when_max_iterations_is_zero() {
        let root = bisection(|x| x - 1.0, 0.0, 2.0, EPSILON, 0);

        assert_eq!(root, None);
    }

    #[test]
    fn returns_none_when_max_iterations_is_insufficient() {
        let root = bisection(|x| x * x - 2.0, 0.0, 2.0, 1e-12, 3);

        assert_eq!(root, None);
    }

    #[test]
    fn returns_midpoint_when_interval_is_already_within_tolerance() {
        let root = bisection(|x| x - 1.0, 0.0, 2.0, 2.0, 100);

        assert_eq!(root, Some(1.0));
    }

    #[test]
    fn finds_root_at_midpoint() {
        let root = bisection(|x| x - 1.0, 0.0, 2.0, 1e-6, 100);

        assert_eq!(root, Some(1.0));
    }
}
