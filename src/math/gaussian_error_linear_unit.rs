//! # Gaussian Error Linear Unit (GELU) Function
//!
//! The `gaussian_error_linear_unit` function computes the Gaussian Error Linear Unit (GELU) values of a given f64 number or a vector of f64 numbers.
//!
//! GELU is an activation function used in neural networks that introduces a smooth approximation of the rectifier function (ReLU).
//! It is defined using the Gaussian cumulative distribution function and can help mitigate the vanishing gradient problem.
//!
//! ## Formula
//!
//! For a given input value `x`, the GELU function computes the output `y` as follows:
//!
//! `y = 0.5 * (1.0 + tanh(2.0 / sqrt(π) * (x + 0.044715 * x^3)))`
//!
//! Where `tanh` is the hyperbolic tangent function and `π` is the mathematical constant (approximately 3.14159).
//!
//! ## Gaussian Error Linear Unit (GELU) Function Implementation
//!
//! This implementation takes either a single f64 value or a reference to a vector of f64 values and returns the GELU transformation applied to each element. The input values are not altered.
//!
use std::f64::consts::E;
use std::f64::consts::PI;

fn tanh(vector: f64) -> f64 {
    (2. / (1. + E.powf(-2. * vector.to_owned()))) - 1.
}

pub fn gaussian_error_linear_unit(vector: &Vec<f64>) -> Vec<f64> {
    let mut gelu_vec = vector.to_owned();
    for value in &mut gelu_vec {
        *value = *value
            * 0.5
            * (1. + tanh(f64::powf(2. / PI, 0.5) * (*value + 0.044715 * value.powf(3.))));
    }

    gelu_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaussian_error_linear_unit() {
        let test_vector = vec![-10., 2., -3., 4., -5., 10., 0.05];
        assert_eq!(
            gaussian_error_linear_unit(&test_vector),
            vec![
                -0.0,
                1.9545976940877752,
                -0.0036373920817729943,
                3.9999297540518075,
                -2.2917961972623857e-7,
                10.0,
                0.025996938238622008
            ]
        );
    }
}
