/// Gradient Descent Optimization
///
/// Gradient descent is an iterative optimization algorithm used to find the minimum of a function.
/// It works by updating the parameters (in this case, elements of the vector `x`) in the direction of
/// the steepest decrease in the function's value. This is achieved by subtracting the gradient of
/// the function at the current point from the current point. The learning rate controls the step size.
///
/// The equation for a single parameter (univariate) is:
/// x_{k+1} = x_k - learning_rate * derivative_of_function(x_k)
///
/// For multivariate functions, it extends to each parameter:
/// x_{k+1} = x_k - learning_rate * gradient_of_function(x_k)
///
/// # Arguments
///
/// * `derivative_fn` - The function that calculates the gradient of the objective function at a given point.
/// * `x` - The initial parameter vector to be optimized.
/// * `learning_rate` - Step size for each iteration.
/// * `num_iterations` - The number of iterations to run the optimization.
///
/// # Returns
///
/// A reference to the optimized parameter vector `x`.

pub fn gradient_descent(
    derivative_fn: fn(&[f64]) -> Vec<f64>,
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
        fn derivative_of_square(params: &[f64]) -> Vec<f64> {
            params.iter().map(|x| 2. * x).collect()
        }

        let mut x: Vec<f64> = vec![5.0, 6.0];
        let learning_rate: f64 = 0.03;
        let num_iterations: i32 = 1000;

        let minimized_vector =
            gradient_descent(derivative_of_square, &mut x, learning_rate, num_iterations);

        let test_vector = [0.0, 0.0];

        let tolerance = 1e-6;
        for (minimized_value, test_value) in minimized_vector.iter().zip(test_vector.iter()) {
            assert!((minimized_value - test_value).abs() < tolerance);
        }
    }

    #[test]
    fn test_gradient_descent_unoptimized() {
        fn derivative_of_square(params: &[f64]) -> Vec<f64> {
            params.iter().map(|x| 2. * x).collect()
        }

        let mut x: Vec<f64> = vec![5.0, 6.0];
        let learning_rate: f64 = 0.03;
        let num_iterations: i32 = 10;

        let minimized_vector =
            gradient_descent(derivative_of_square, &mut x, learning_rate, num_iterations);

        let test_vector = [0.0, 0.0];

        let tolerance = 1e-6;
        for (minimized_value, test_value) in minimized_vector.iter().zip(test_vector.iter()) {
            assert!((minimized_value - test_value).abs() >= tolerance);
        }
    }
}
