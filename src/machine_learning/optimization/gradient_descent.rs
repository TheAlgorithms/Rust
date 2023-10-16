pub fn gradient_descent(
    derivative_fn: fn(&Vec<f64>) -> Vec<f64>,
    x: &mut Vec<f64>,
    learning_rate: f64,
    num_iterations: i32,
) -> &mut Vec<f64> {
    for _ in 0..num_iterations {
        let gradient = derivative_fn(x);
        for (x_k, grad) in x.iter_mut().zip(gradient.iter()) {
            *x_k -= learning_rate * grad;
        }
    }

    x
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gradient_descent_optimized() {
        fn derivative_of_square(params: &Vec<f64>) -> Vec<f64> {
            params.iter().map(|x| 2. * x).collect()
        }

        let mut x: Vec<f64> = vec![5.0, 6.0];
        let learning_rate: f64 = 0.03;
        let num_iterations: i32 = 1000;

        let minimized_vector =
            gradient_descent(derivative_of_square, &mut x, learning_rate, num_iterations);

        let test_vector = vec![0.0, 0.0];

        let tolerance = 1e-6;
        for (minimized_value, test_value) in minimized_vector.iter().zip(test_vector.iter()) {
            assert!((minimized_value - test_value).abs() < tolerance);
        }
    }

    #[test]
    fn test_gradient_descent_unoptimized() {
        fn derivative_of_square(params: &Vec<f64>) -> Vec<f64> {
            params.iter().map(|x| 2. * x).collect()
        }

        let mut x: Vec<f64> = vec![5.0, 6.0];
        let learning_rate: f64 = 0.03;
        let num_iterations: i32 = 10;

        let minimized_vector =
            gradient_descent(derivative_of_square, &mut x, learning_rate, num_iterations);

        let test_vector = vec![0.0, 0.0];

        let tolerance = 1e-6;
        for (minimized_value, test_value) in minimized_vector.iter().zip(test_vector.iter()) {
            assert_eq!((minimized_value - test_value).abs() < tolerance, false);
        }
    }
}
