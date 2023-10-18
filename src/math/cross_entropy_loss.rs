//! # Cross-Entropy Loss Function
//!
//! The `cross_entropy_loss` function calculates the cross-entropy loss between the actual and predicted probability distributions.
//!
//! Cross-entropy loss is commonly used in machine learning and deep learning to measure the dissimilarity between two probability distributions. It is often used in classification problems.
//!
//! ## Formula
//!
//! For a pair of actual and predicted probability distributions represented as vectors `actual` and `predicted`, the cross-entropy loss is calculated as:
//!
//! `L = -Σ(actual[i] * ln(predicted[i]))` for all `i` in the range of the vectors
//!
//! Where `ln` is the natural logarithm function, and `Σ` denotes the summation over all elements of the vectors.
//!
//! ## Cross-Entropy Loss Function Implementation
//!
//! This implementation takes two references to vectors of f64 values, `actual` and `predicted`, and returns the cross-entropy loss between them.
//!
pub fn cross_entropy_loss(actual: &[f64], predicted: &[f64]) -> f64 {
    let mut loss: Vec<f64> = Vec::new();
    for (a, p) in actual.iter().zip(predicted.iter()) {
        loss.push(-a * p.ln());
    }
    loss.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_entropy_loss() {
        let test_vector_actual = vec![0., 1., 0., 0., 0., 0.];
        let test_vector = vec![0.1, 0.7, 0.1, 0.05, 0.05, 0.1];
        assert_eq!(
            cross_entropy_loss(&test_vector_actual, &test_vector),
            0.35667494393873245
        );
    }
}
