// This gives a better approximation than naive approach
// See https://en.wikipedia.org/wiki/Simpson%27s_rule
pub fn simpson_integration<F: Fn(f64) -> f64>(
    start: f64,
    end: f64,
    steps: u64,
    function: F,
) -> f64 {
    let mut result = function(start) + function(end);
    let step = (end - start) / steps as f64;
    for i in 1..steps {
        let x = start + step * i as f64;
        match i % 2 {
            0 => result += function(x) * 2.0,
            1 => result += function(x) * 4.0,
            _ => unreachable!(),
        }
    }
    result *= step / 3.0;
    result
}

#[cfg(test)]
mod tests {

    use super::*;
    const EPSILON: f64 = 1e-9;

    fn almost_equal(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    #[test]
    fn parabola_curve_length() {
        // Calculate the length of the curve f(x) = x^2 for -5 <= x <= 5
        // We should integrate sqrt(1 + (f'(x))^2)
        let function = |x: f64| -> f64 { (1.0 + 4.0 * x * x).sqrt() };
        let result = simpson_integration(-5.0, 5.0, 1_000, function);
        let integrated = |x: f64| -> f64 { (x * function(x) / 2.0) + ((2.0 * x).asinh() / 4.0) };
        let expected = integrated(5.0) - integrated(-5.0);
        assert!(almost_equal(result, expected, EPSILON));
    }

    #[test]
    fn area_under_cosine() {
        use std::f64::consts::PI;
        // Calculate area under f(x) = cos(x) + 5 for -pi <= x <= pi
        // cosine should cancel out and the answer should be 2pi * 5
        let function = |x: f64| -> f64 { x.cos() + 5.0 };
        let result = simpson_integration(-PI, PI, 1_000, function);
        let expected = 2.0 * PI * 5.0;
        assert!(almost_equal(result, expected, EPSILON));
    }
}
