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

    macro_rules! test_mrg_ranking_loss {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (x_first, x_second, margin, y_true, expected) = $test_case;
                    let result = mrg_ranking_loss(&x_first, &x_second, margin, y_true);
                    assert_eq!(result, expected);
                }
            )*
        };
    }

    test_mrg_ranking_loss! {
        test_simple_ranking_example: (vec![3.0, 5.0, 2.0], vec![2.0, 4.0, 1.0], 1.0, 1.0, Some(0.0)),
        test_negative_margin: (vec![1.0, 2.0, 3.0], vec![3.0, 2.0, 1.0], 0.5, -1.0, Some(1.0)),
        test_identical_scores: (vec![1.0, 1.0, 1.0], vec![1.0, 1.0, 1.0], 1.0, 1.0, Some(1.0)),
        test_mixed_y_true: (vec![3.0, 5.0, 7.0], vec![2.0, 6.0, 1.0], 1.0, -1.0, Some(3.0)),
        test_different_lengths: (vec![1.0, 2.0], vec![3.0], 1.0, 1.0, None),
        test_empty_vectors: (vec![], vec![], 1.0, 1.0, None),
    }
}
