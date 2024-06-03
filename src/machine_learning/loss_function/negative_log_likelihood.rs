// Negative Log Likelihood Loss Function
//
// The `neg_log_likelihood` function calculates the Negative Log Likelyhood loss,
// which is a loss function used for classification problems in machine learning.
//
// ## Formula
//
// For a pair of actual and predicted values, represented as vectors `y_true` and
// `y_pred`, the Negative Log Likelihood loss is calculated as:
//
// - loss = `-y_true * log(y_pred) - (1 - y_true) * log(1 - y_pred)`.
//
// It returns the average loss by dividing the `total_loss` by total no. of
// elements.

pub fn neg_log_likelihood(y_true: &[f64], y_pred: &[f64]) -> f64 {
    let mut total_loss: f64 = 0.0;
    for (p, a) in y_pred.iter().zip(y_true.iter()) {
        let loss: f64 = -a * p.ln() - (1.0 - a) * (1.0 - p).ln();
        total_loss += loss;
    }
    total_loss / (y_pred.len() as f64)
}
