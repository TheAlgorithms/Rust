pub fn trapezoidal_integral<F>(a: f64, b: f64, f: F, precision: u32) -> f64
where
    F: Fn(f64) -> f64,
{
    let delta = (b - a) / precision as f64;

    (0..precision)
        .map(|trapezoid| {
            let left_side = a + (delta * trapezoid as f64);
            let right_side = left_side + delta;

            0.5 * (f(left_side) + f(right_side)) * delta
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_trapezoidal_integral {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (a, b, f, prec, expected, eps) = $inputs;
                let actual = trapezoidal_integral(a, b, f, prec);
                assert!((actual - expected).abs() < eps);
            }
        )*
        }
    }

    test_trapezoidal_integral! {
        basic_0: (0.0, 1.0, |x: f64| x.powi(2), 1000, 1.0/3.0, 0.0001),
        basic_0_higher_prec: (0.0, 1.0, |x: f64| x.powi(2), 10000, 1.0/3.0, 0.00001),
        basic_1: (-1.0, 1.0, |x: f64| x.powi(2), 10000, 2.0/3.0, 0.00001),
        basic_1_higher_prec: (-1.0, 1.0, |x: f64| x.powi(2), 100000, 2.0/3.0, 0.000001),
        flipped_limits: (1.0, 0.0, |x: f64| x.powi(2), 10000, -1.0/3.0, 0.00001),
        empty_range: (0.5, 0.5, |x: f64| x.powi(2), 100, 0.0, 0.0000001),
    }
}
