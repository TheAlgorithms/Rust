//! # Hinge Loss
//!
//! The `hng_loss` function calculates the Hinge loss, which is a
//! loss function used for classification problems in machine learning.
//!
//! ## Formula
//!
//! For a pair of actual and predicted values, represented as vectors `y_true` and
//! `y_pred`, the Hinge loss is calculated as:
//!
//! - loss = `max(0, 1 - y_true * y_pred)`.
//!
//! It returns the average loss by dividing the `total_loss` by total no. of
//! elements.
//!
pub fn hng_loss(y_true: &[f64], y_pred: &[f64]) -> f64 {
    let mut total_loss: f64 = 0.0;
    for (p, a) in y_pred.iter().zip(y_true.iter()) {
        let loss: f64 = (1.0 - a * p).max(0.0);
        total_loss += loss;
    }
    total_loss / (y_pred.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hinge_loss() {
        let predicted_values: Vec<f64> = vec![-1.0, 1.0, 1.0];
        let actual_values: Vec<f64> = vec![-1.0, -1.0, 1.0];
        assert_eq!(
            hng_loss(&predicted_values, &actual_values),
            0.6666666666666666
        );
    }
}
