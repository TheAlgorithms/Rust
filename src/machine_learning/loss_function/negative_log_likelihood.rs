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
//
// https://towardsdatascience.com/cross-entropy-negative-log-likelihood-and-all-that-jazz-47a95bd2e81

pub fn neg_log_likelihood(
    y_true: &[f64],
    y_pred: &[f64],
) -> Result<f64, NegativeLogLikelihoodLossError> {
    // Checks if the length of the actual and predicted values are equal
    if y_true.len() != y_pred.len() {
        return Err(NegativeLogLikelihoodLossError::InputsHaveDifferentLength);
    }
    // Checks if the inputs are empty
    if y_pred.is_empty() {
        return Err(NegativeLogLikelihoodLossError::EmptyInputs);
    }
    // Checks values are between 0 and 1
    if !are_all_values_in_range(y_true) || !are_all_values_in_range(y_pred) {
        return Err(NegativeLogLikelihoodLossError::InvalidValues);
    }

    let mut total_loss: f64 = 0.0;
    for (p, a) in y_pred.iter().zip(y_true.iter()) {
        let loss: f64 = -a * p.ln() - (1.0 - a) * (1.0 - p).ln();
        total_loss += loss;
    }
    Ok(total_loss / (y_pred.len() as f64))
}

#[derive(Debug, PartialEq, Eq)]
pub enum NegativeLogLikelihoodLossError {
    InputsHaveDifferentLength,
    EmptyInputs,
    InvalidValues,
}

fn are_all_values_in_range(values: &[f64]) -> bool {
    values.iter().all(|&x| (0.0..=1.0).contains(&x))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_neg_log_likelihood() {
        let actual_values: Vec<f64> = vec![1.0, 0.0, 1.0];
        let predicted_values: Vec<f64> = vec![0.9, 0.1, 0.8];
        assert_eq!(
            neg_log_likelihood(&actual_values, &predicted_values),
            Ok(0.14462152754328741)
        );
    }

    #[test]
    fn test_neg_log_likelihood_invalid_length0() {
        let actual_values: Vec<f64> = vec![1.0, 0.0, 1.0];
        let predicted_values: Vec<f64> = vec![0.9, 0.1];
        assert_eq!(
            neg_log_likelihood(&actual_values, &predicted_values),
            Err(NegativeLogLikelihoodLossError::InputsHaveDifferentLength)
        );
    }

    #[test]
    fn test_neg_log_likelihood_invalid_length1() {
        let actual_values: Vec<f64> = vec![1.0, 0.0];
        let predicted_values: Vec<f64> = vec![0.9, 0.1, 0.8];
        assert_eq!(
            neg_log_likelihood(&actual_values, &predicted_values),
            Err(NegativeLogLikelihoodLossError::InputsHaveDifferentLength)
        );
    }

    #[test]
    fn test_neg_log_likelihood_invalid_values() {
        let actual_values: Vec<f64> = vec![1.0, 0.0, 1.0];
        let predicted_values: Vec<f64> = vec![1.1, 0.1, 0.8];
        assert_eq!(
            neg_log_likelihood(&actual_values, &predicted_values),
            Err(NegativeLogLikelihoodLossError::InvalidValues)
        );
    }

    #[test]
    fn test_neg_log_likelihood_empty_prediction() {
        let actual_values: Vec<f64> = vec![1.0, 0.0, 1.0];
        let predicted_values: Vec<f64> = vec![];
        assert_eq!(
            neg_log_likelihood(&actual_values, &predicted_values),
            Err(NegativeLogLikelihoodLossError::EmptyInputs)
        );
    }

    #[test]
    fn test_neg_log_likelihood_negative_values0() {
        let actual_values: Vec<f64> = vec![-1.0, 0.0, 1.0];
        let predicted_values: Vec<f64> = vec![0.9, 0.1, 0.8];
        assert_eq!(
            neg_log_likelihood(&actual_values, &predicted_values),
            Err(NegativeLogLikelihoodLossError::InvalidValues)
        );
    }

    #[test]
    fn test_neg_log_likelihood_negative_values1() {
        let actual_values: Vec<f64> = vec![1.0, 0.0, 1.0];
        let predicted_values: Vec<f64> = vec![0.9, 0.1, -0.8];
        assert_eq!(
            neg_log_likelihood(&actual_values, &predicted_values),
            Err(NegativeLogLikelihoodLossError::InvalidValues)
        );
    }
}
