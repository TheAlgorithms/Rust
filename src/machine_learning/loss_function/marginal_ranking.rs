// Marginal Ranking
//
// The 'mrg_ranking_loss' function calculates the Marginal Ranking loss, which is a
// loss function used for ranking problems in machine learning.
//
// ## Formula
//
// For a pair of values `x_first` and `x_second`, `margin`, and `y_true`,
// the Marginal Ranking loss is calculated as:
//
//  - loss = `max(0, -y_true * (x_first - x_second) + margin)`.
//
// It returns the average loss by dividing the `total_loss` by total no. of
// elements.

pub fn mrg_ranking_loss(
    x_first: &[f64],
    x_second: &[f64],
    margin: f64,
    y_true: f64,
) -> Option<f64> {
    if x_first.len() != x_second.len() || x_first.is_empty() || x_second.is_empty() {
        return None;
    }
    if margin < 0.0 {
        return None;
    }
    if y_true != 1.0 && y_true != -1.0 {
        return None;
    }

    let mut total_loss: f64 = 0.0;
    for (f, s) in x_first.iter().zip(x_second.iter()) {
        let loss: f64 = (margin - y_true * (f - s)).max(0.0);
        total_loss += loss;
    }
    Some(total_loss / (x_first.len() as f64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marginal_ranking_loss() {
        let first_values: Vec<f64> = vec![1.0, 2.0, 3.0];
        let second_values: Vec<f64> = vec![2.0, 3.0, 4.0];
        let margin: f64 = 1.0;
        let actual_value: f64 = -1.0;
        assert_eq!(
            mrg_ranking_loss(&first_values, &second_values, margin, actual_value),
            Some(0.0)
        );
    }

    #[test]
    fn test_marginal_ranking_loss_invalid_length0() {
        let x_first: Vec<f64> = vec![1.0, 2.0, 3.0];
        let x_second: Vec<f64> = vec![2.0, 3.0];
        let margin: f64 = 1.0;
        let y_true: f64 = 1.0;
        assert_eq!(mrg_ranking_loss(&x_first, &x_second, margin, y_true), None);
    }

    #[test]
    fn test_marginal_ranking_loss_invalid_length1() {
        let x_first: Vec<f64> = vec![1.0, 2.0];
        let x_second: Vec<f64> = vec![2.0, 3.0, 4.0];
        let margin: f64 = 1.0;
        let y_true: f64 = 1.0;
        assert_eq!(mrg_ranking_loss(&x_first, &x_second, margin, y_true), None);
    }

    #[test]
    fn test_marginal_ranking_invalid_values() {
        let x_first: Vec<f64> = vec![1.0, 2.0, 3.0];
        let x_second: Vec<f64> = vec![2.0, 3.0, 4.0];
        let margin: f64 = -1.0;
        let y_true: f64 = 1.0;
        assert_eq!(mrg_ranking_loss(&x_first, &x_second, margin, y_true), None);
    }

    #[test]
    fn test_marginal_ranking_invalid_y_true() {
        let x_first: Vec<f64> = vec![1.0, 2.0, 3.0];
        let x_second: Vec<f64> = vec![2.0, 3.0, 4.0];
        let margin: f64 = 1.0;
        let y_true: f64 = 2.0;
        assert_eq!(mrg_ranking_loss(&x_first, &x_second, margin, y_true), None);
    }

    #[test]
    fn test_marginal_ranking_empty_prediction0() {
        let x_first: Vec<f64> = vec![];
        let x_second: Vec<f64> = vec![1.0, 2.0, 3.0];
        let margin: f64 = 1.0;
        let y_true: f64 = 1.0;
        assert_eq!(mrg_ranking_loss(&x_first, &x_second, margin, y_true), None);
    }

    #[test]
    fn test_marginal_ranking_empty_prediction1() {
        let x_first: Vec<f64> = vec![1.0, 2.0, 3.0];
        let x_second: Vec<f64> = vec![];
        let margin: f64 = 1.0;
        let y_true: f64 = 1.0;
        assert_eq!(mrg_ranking_loss(&x_first, &x_second, margin, y_true), None);
    }
}
