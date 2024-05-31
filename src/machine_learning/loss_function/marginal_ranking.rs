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
