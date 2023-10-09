//! # Softmax Function
//!
//! The `softmax` function computes the softmax values of a given array of f32 numbers.
//!
//! The softmax operation is often used in machine learning for converting a vector of real numbers into a
//! probability distribution. It exponentiates each element in the input array, and then normalizes the
//! results so that they sum to 1.
//!
//! ## Formula
//!
//! For a given input array `x`, the softmax function computes the output `y` as follows:
//!
//! `y_i = e^(x_i) / sum(e^(x_j) for all j)`
//!
//! ## Softmax Function Implementation
//!
//! This implementation uses the `std::f32::consts::E` constant for the base of the exponential function. and
//! f32 vectors to compute the values. The function creates a new vector and not altering the input vector.
//!
use std::f32::consts::E;

pub fn softmax(array: Vec<f32>) -> Vec<f32> {
    let mut softmax_array = array.clone();

    for value in &mut softmax_array {
        *value = E.powf(*value);
    }

    let sum: f32 = softmax_array.iter().sum();

    for value in &mut softmax_array {
        *value /= sum;
    }

    softmax_array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_softmax() {
        let test = vec![9.0, 0.5, -3.0, 0.0, 3.0];
        assert_eq!(
            softmax(test),
            vec![
                0.9971961,
                0.00020289792,
                6.126987e-6,
                0.00012306382,
                0.0024718025
            ]
        );
    }
}
