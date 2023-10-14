//! # Exponential Linear Unit (ELU) Function
//!
//! The `exponential_linear_unit` function computes the Exponential Linear Unit (ELU) values of a given vector
//! of f64 numbers with a specified alpha parameter.
//!
//! The ELU activation function is commonly used in neural networks as an alternative to the Leaky ReLU function.
//! It introduces a small negative slope (controlled by the alpha parameter) for the negative input values and has
//! an exponential growth for positive values, which can help mitigate the vanishing gradient problem.
//!
//! ## Formula
//!
//! For a given input vector `x` and an alpha parameter `alpha`, the ELU function computes the output
//! `y` as follows:
//!
//! `y_i = { x_i if x_i >= 0, alpha * (e^x_i - 1) if x_i < 0 }`
//!
//! Where `e` is the mathematical constant (approximately 2.71828).
//!
//! ## Exponential Linear Unit (ELU) Function Implementation
//!
//! This implementation takes a reference to a vector of f64 values and an alpha parameter, and returns a new
//! vector with the ELU transformation applied to each element. The input vector is not altered.
//!

use std::f64::consts::E;

pub fn exponential_linear_unit(vector: &Vec<f64>, alpha: f64) -> Vec<f64> {
    let mut _vector = vector.to_owned();

    for value in &mut _vector {
        if value < &mut 0. {
            *value *= alpha * (E.powf(*value) - 1.);
        }
    }

    _vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_linear_unit() {
        let test_vector = vec![-10., 2., -3., 4., -5., 10., 0.05];
        let alpha = 0.01;
        assert_eq!(
            exponential_linear_unit(&test_vector, alpha),
            vec![
                0.09999546000702375,
                2.0,
                0.028506387948964082,
                4.0,
                0.049663102650045726,
                10.0,
                0.05
            ]
        );
    }
}
