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

    macro_rules! huber_loss_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (y_true, y_pred, delta, expected_loss) = $test_case;
                    assert_eq!(huber_loss(&y_true, &y_pred, delta), expected_loss);
                }
            )*
        };
    }

    huber_loss_tests! {
        test_huber_loss_residual_less_than_delta: (
            vec![10.0, 8.0, 12.0],
            vec![9.0, 7.0, 11.0],
            1.0,
            Some(0.5)
        ),
        test_huber_loss_residual_greater_than_delta: (
            vec![3.0, 5.0, 7.0],
            vec![2.0, 4.0, 8.0],
            0.5,
            Some(0.375)
        ),
        test_huber_loss_invalid_length: (
            vec![10.0, 8.0, 12.0],
            vec![7.0, 6.0],
            1.0,
            None
        ),
        test_huber_loss_empty_prediction: (
            vec![10.0, 8.0, 12.0],
            vec![],
            1.0,
            None
        ),
    }
}
