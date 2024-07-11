/// Marginal Ranking
///
/// The 'mrg_ranking_loss' function calculates the Marginal Ranking loss, which is a
/// loss function used for ranking problems in machine learning.
///
/// ## Formula
///
/// For a pair of values `x_first` and `x_second`, `margin`, and `y_true`,
/// the Marginal Ranking loss is calculated as:
///
///  - loss = `max(0, -y_true * (x_first - x_second) + margin)`.
///
/// It returns the average loss by dividing the `total_loss` by total no. of
/// elements.
///
/// Pytorch implementation:
/// https://pytorch.org/docs/stable/generated/torch.nn.MarginRankingLoss.html
/// https://gombru.github.io/2019/04/03/ranking_loss/
/// https://vinija.ai/concepts/loss/#pairwise-ranking-loss
///

pub fn mrg_ranking_loss(
    x_first: &[f64],
    x_second: &[f64],
    margin: f64,
    y_true: f64,
) -> Result<f64, MarginalRankingLossError> {
    if x_first.len() != x_second.len() {
        return Err(MarginalRankingLossError::InputsHaveDifferentLength);
    }
    if x_first.is_empty() {
        return Err(MarginalRankingLossError::EmptyInputs);
    }
    if margin < 0.0 {
        return Err(MarginalRankingLossError::InvalidValues);
    }
    if y_true != 1.0 && y_true != -1.0 {
        return Err(MarginalRankingLossError::InvalidValues);
    }

    let mut total_loss: f64 = 0.0;
    for (f, s) in x_first.iter().zip(x_second.iter()) {
        let loss: f64 = (margin - y_true * (f - s)).max(0.0);
        total_loss += loss;
    }
    Ok(total_loss / (x_first.len() as f64))
}

#[derive(Debug, PartialEq, Eq)]
pub enum MarginalRankingLossError {
    InputsHaveDifferentLength,
    EmptyInputs,
    InvalidValues,
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_with_wrong_inputs {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (x_first, x_second, margin, y_true, expected) = $inputs;
                    assert_eq!(mrg_ranking_loss(&x_first, &x_second, margin, y_true), expected);
                }
            )*
        }
    }

    test_with_wrong_inputs! {
        invalid_length0: (vec![1.0, 2.0, 3.0], vec![2.0, 3.0], 1.0, 1.0, Err(MarginalRankingLossError::InputsHaveDifferentLength)),
        invalid_length1: (vec![1.0, 2.0], vec![2.0, 3.0, 4.0], 1.0, 1.0, Err(MarginalRankingLossError::InputsHaveDifferentLength)),
        invalid_length2: (vec![], vec![1.0, 2.0, 3.0], 1.0, 1.0, Err(MarginalRankingLossError::InputsHaveDifferentLength)),
        invalid_length3: (vec![1.0, 2.0, 3.0], vec![], 1.0, 1.0, Err(MarginalRankingLossError::InputsHaveDifferentLength)),
        invalid_values: (vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0], -1.0, 1.0, Err(MarginalRankingLossError::InvalidValues)),
        invalid_y_true: (vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0], 1.0, 2.0, Err(MarginalRankingLossError::InvalidValues)),
        empty_inputs: (vec![], vec![], 1.0, 1.0, Err(MarginalRankingLossError::EmptyInputs)),
    }

    macro_rules! test_marginal_ranking_loss {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (x_first, x_second, margin, y_true, expected) = $inputs;
                    assert_eq!(mrg_ranking_loss(&x_first, &x_second, margin, y_true), Ok(expected));
                }
            )*
        }
    }

    test_marginal_ranking_loss! {
        set_0: (vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0], 1.0, -1.0, 0.0),
        set_1: (vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0], 1.0, 1.0, 2.0),
        set_2: (vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0], 0.0, 1.0, 0.0),
        set_3: (vec![4.0, 5.0, 6.0], vec![1.0, 2.0, 3.0], 1.0, -1.0, 4.0),
    }
}
