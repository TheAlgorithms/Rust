// Gaussian Quadrature Module
//https://en.wikipedia.org/wiki/Gaussian_quadrature

pub fn gaussian_quadrature(a: f64, b: f64, f: impl Fn(f64) -> f64, order: usize) -> f64 {
    let (points, weights) = get_gaussian_quadrature_points_weights(num_points);

    let mut result = 0.0;

    for i in 0..num_points {
        let x_i = 0.5 * (a + b) + 0.5 * (b - a) * points[i];
        result += weights[i] * f(x_i);
    }

    0.5 * (b - a) * result
}

fn get_gaussian_quadrature_points_weights(num_points: usize) -> (Vec<f64>, Vec<f64>) {
    // Hardcoded values for Gaussian Quadrature points and weights
    match num_points {
        1 => (vec![0.0], vec![2.0]),
        2 => (vec![-1.0 / 3.0, 1.0 / 3.0], vec![1.0, 1.0]),
        3 => (
            vec![-0.7745966692414834, 0.0, 0.7745966692414834],
            vec![0.5555555555555556, 0.8888888888888888, 0.5555555555555556],
        ),
        _ => unimplemented!("Gaussian Quadrature not implemented for this number of points"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaussian_quadrature_basic() {
        let a = 0.0;
        let b = 1.0;
        let f = |x: f64| x.powi(2);
        let num_points = 3;
        let expected = 1.0 / 3.0;
        let eps = 0.0001;

        let result = gaussian_quadrature(a, b, f, num_points);
        assert!((result - expected).abs() < eps);
    }
}
