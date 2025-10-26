/// Momentum Optimization
///
/// Momentum is an extension of gradient descent that accelerates convergence by accumulating
/// a velocity vector in directions of persistent reduction in the objective function.
/// This helps the optimizer navigate ravines and avoid getting stuck in local minima.
///
/// The algorithm maintains a velocity vector that accumulates exponentially decaying moving
/// averages of past gradients. This allows the optimizer to build up speed in consistent
/// directions while dampening oscillations.
///
/// The update equations are:
/// velocity_{k+1} = beta * velocity_k + gradient_of_function(x_k)
/// x_{k+1} = x_k - learning_rate * velocity_{k+1}
///
/// where beta (typically 0.9) controls how much past gradients influence the current update.
///
/// # Arguments
///
/// * `derivative_fn` - The function that calculates the gradient of the objective function at a given point.
/// * `x` - The initial parameter vector to be optimized.
/// * `learning_rate` - Step size for each iteration.
/// * `beta` - Momentum coefficient (typically 0.9). Higher values give more weight to past gradients.
/// * `num_iterations` - The number of iterations to run the optimization.
///
/// # Returns
///
/// A reference to the optimized parameter vector `x`.
pub fn momentum(
    derivative: impl Fn(&[f64]) -> Vec<f64>,
    x: &mut Vec<f64>,
    learning_rate: f64,
    beta: f64,
    num_iterations: i32,
) -> &mut Vec<f64> {
    // Initialize velocity vector to zero
    let mut velocity: Vec<f64> = vec![0.0; x.len()];

    for _ in 0..num_iterations {
        let gradient = derivative(x);

        // Update velocity and parameters
        for ((x_k, vel), grad) in x.iter_mut().zip(velocity.iter_mut()).zip(gradient.iter()) {
            *vel = beta * *vel + grad;
            *x_k -= learning_rate * *vel;
        }
    }
    x
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_momentum_optimized() {
        fn derivative_of_square(params: &[f64]) -> Vec<f64> {
            params.iter().map(|x| 2.0 * x).collect()
        }

        let mut x: Vec<f64> = vec![5.0, 6.0];
        let learning_rate: f64 = 0.01;
        let beta: f64 = 0.9;
        let num_iterations: i32 = 1000;

        let minimized_vector = momentum(
            derivative_of_square,
            &mut x,
            learning_rate,
            beta,
            num_iterations,
        );

        let test_vector = [0.0, 0.0];
        let tolerance = 1e-6;

        for (minimized_value, test_value) in minimized_vector.iter().zip(test_vector.iter()) {
            assert!((minimized_value - test_value).abs() < tolerance);
        }
    }

    #[test]
    fn test_momentum_unoptimized() {
        fn derivative_of_square(params: &[f64]) -> Vec<f64> {
            params.iter().map(|x| 2.0 * x).collect()
        }

        let mut x: Vec<f64> = vec![5.0, 6.0];
        let learning_rate: f64 = 0.01;
        let beta: f64 = 0.9;
        let num_iterations: i32 = 10;

        let minimized_vector = momentum(
            derivative_of_square,
            &mut x,
            learning_rate,
            beta,
            num_iterations,
        );

        let test_vector = [0.0, 0.0];
        let tolerance = 1e-6;

        for (minimized_value, test_value) in minimized_vector.iter().zip(test_vector.iter()) {
            assert!((minimized_value - test_value).abs() >= tolerance);
        }
    }

    #[test]
    fn test_momentum_faster_than_gd() {
        fn derivative_of_square(params: &[f64]) -> Vec<f64> {
            params.iter().map(|x| 2.0 * x).collect()
        }

        // Test that momentum converges faster than gradient descent
        let mut x_momentum: Vec<f64> = vec![5.0, 6.0];
        let mut x_gd: Vec<f64> = vec![5.0, 6.0];
        let learning_rate: f64 = 0.01;
        let beta: f64 = 0.9;
        let num_iterations: i32 = 50;

        momentum(
            derivative_of_square,
            &mut x_momentum,
            learning_rate,
            beta,
            num_iterations,
        );

        // Gradient descent from your original implementation
        for _ in 0..num_iterations {
            let gradient = derivative_of_square(&x_gd);
            for (x_k, grad) in x_gd.iter_mut().zip(gradient.iter()) {
                *x_k -= learning_rate * grad;
            }
        }

        // Momentum should be closer to zero
        let momentum_distance: f64 = x_momentum.iter().map(|x| x * x).sum();
        let gd_distance: f64 = x_gd.iter().map(|x| x * x).sum();

        assert!(momentum_distance < gd_distance);
    }
}
