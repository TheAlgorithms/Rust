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
