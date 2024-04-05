/// Computes the Huber loss between arrays of true and predicted values.
///
/// # Arguments
///
/// * `y_true` - An array of true values.
/// * `y_pred` - An array of predicted values.
/// * `delta` - The threshold parameter that controls the linear behavior of the loss function.
///
/// # Returns
///
/// The average Huber loss for all pairs of true and predicted values.
pub fn huber_loss(y_true: &[f64], y_pred: &[f64], delta: f64) -> Option<f64> {
    if y_true.len() != y_pred.len() || y_pred.is_empty() {
        return None;
    }

    let loss: f64 = y_true
        .iter()
        .zip(y_pred.iter())
        .map(|(&true_val, &pred_val)| {
            let residual = (true_val - pred_val).abs();
            match residual {
                r if r <= delta => 0.5 * r.powi(2),
                _ => delta * residual - 0.5 * delta.powi(2),
            }
        })
        .sum();

    Some(loss / (y_pred.len() as f64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huber_loss_residual_less_than_delta() {
        let y_true = vec![10.0, 8.0, 12.0];
        let y_pred = vec![9.0, 7.0, 11.0];
        let delta = 1.0;
        let expected_loss = 0.5;
        assert_eq!(huber_loss(&y_true, &y_pred, delta), Some(expected_loss));
    }

    #[test]
    fn test_huber_loss_residual_greater_than_delta() {
        let y_true = vec![3.0, 5.0, 7.0];
        let y_pred = vec![2.0, 4.0, 8.0];
        let delta = 0.5;
        let expected_loss = 0.375;
        assert_eq!(huber_loss(&y_true, &y_pred, delta), Some(expected_loss));
    }

    #[test]
    fn test_huber_loss_invalid_length() {
        let y_true = vec![10.0, 8.0, 12.0];
        let y_pred = vec![7.0, 6.0];
        let delta = 1.0;
        assert_eq!(huber_loss(&y_true, &y_pred, delta), None);
    }

    #[test]
    fn test_huber_loss_empty_prediction() {
        let y_true = vec![10.0, 8.0, 12.0];
        let y_pred = vec![];
        let delta = 1.0;
        assert_eq!(huber_loss(&y_true, &y_pred, delta), None);
    }
}
