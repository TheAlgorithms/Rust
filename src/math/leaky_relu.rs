//! # Leaky ReLU Function
//!
//! The `leaky_relu` function computes the Leaky Rectified Linear Unit (ReLU) values of a given vector
//! of f64 numbers with a specified alpha parameter.
//!
//! The Leaky ReLU activation function is commonly used in neural networks to introduce a small negative
//! slope (controlled by the alpha parameter) for the negative input values, preventing neurons from dying
//! during training.
//!
//! ## Formula
//!
//! For a given input vector `x` and an alpha parameter `alpha`, the Leaky ReLU function computes the output
//! `y` as follows:
//!
//! `y_i = { x_i if x_i >= 0, alpha * x_i if x_i < 0 }`
//!
//! ## Leaky ReLU Function Implementation
//!
//! This implementation takes a reference to a vector of f64 values and an alpha parameter, and returns a new
//! vector with the Leaky ReLU transformation applied to each element. The input vector is not altered.
//!
pub fn leaky_relu(vector: &Vec<f64>, alpha: f64) -> Vec<f64> {
    let mut _vector = vector.to_owned();

    for value in &mut _vector {
        if value < &mut 0. {
            *value *= alpha;
        }
    }

    _vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaky_relu() {
        let test_vector = vec![-10., 2., -3., 4., -5., 10., 0.05];
        let alpha = 0.01;
        assert_eq!(
            leaky_relu(&test_vector, alpha),
            vec![-0.1, 2.0, -0.03, 4.0, -0.05, 10.0, 0.05]
        );
    }
}
