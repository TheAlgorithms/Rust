// Author : cyrixninja
// Wikipedia : https://en.wikipedia.org/wiki/Geometric_series
// Calculate a geometric series.

pub fn geometric_series(nth_term: f64, start_term_a: f64, common_ratio_r: f64) -> Vec<f64> {
    let mut series = Vec::new();
    let mut multiple = 1.0;

    for _ in 0..(nth_term as i32) {
        series.push(start_term_a * multiple);
        multiple *= common_ratio_r;
    }

    series
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_approx_eq(a: f64, b: f64) {
        let epsilon = 1e-10;
        assert!((a - b).abs() < epsilon, "Expected {a}, found {b}");
    }

    #[test]
    fn test_geometric_series() {
        let result = geometric_series(4.0, 2.0, 2.0);
        assert_eq!(result.len(), 4);
        assert_approx_eq(result[0], 2.0);
        assert_approx_eq(result[1], 4.0);
        assert_approx_eq(result[2], 8.0);
        assert_approx_eq(result[3], 16.0);

        let result = geometric_series(4.1, 2.1, 2.1);
        assert_eq!(result.len(), 4);
        assert_approx_eq(result[0], 2.1);
        assert_approx_eq(result[1], 4.41);
        assert_approx_eq(result[2], 9.261);
        assert_approx_eq(result[3], 19.4481);

        let result = geometric_series(4.0, -2.0, 2.0);
        assert_eq!(result.len(), 4);
        assert_approx_eq(result[0], -2.0);
        assert_approx_eq(result[1], -4.0);
        assert_approx_eq(result[2], -8.0);
        assert_approx_eq(result[3], -16.0);
    }
}
