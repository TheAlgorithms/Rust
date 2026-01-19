/// Returns the weights and bias after performing Perceptron algorithm on the input data points.
/// The Perceptron is a binary classification algorithm that learns a linear separator.
/// Labels should be either -1.0 or 1.0 for the two classes.
pub fn perceptron(
    data_points: Vec<(Vec<f64>, f64)>,
    max_iterations: usize,
    learning_rate: f64,
) -> Option<(Vec<f64>, f64)> {
    if data_points.is_empty() {
        return None;
    }

    let num_features = data_points[0].0.len();
    if num_features == 0 {
        return None;
    }

    let mut weights = vec![0.0; num_features];
    let mut bias = 0.0;

    for _ in 0..max_iterations {
        let mut misclassified = 0;

        for (features, label) in &data_points {
            let prediction = predict(&weights, bias, features);

            if prediction != *label {
                misclassified += 1;

                for (weight, feature) in weights.iter_mut().zip(features.iter()) {
                    *weight += learning_rate * label * feature;
                }
                bias += learning_rate * label;
            }
        }

        if misclassified == 0 {
            break;
        }
    }

    Some((weights, bias))
}

/// Make a prediction using the given weights and bias.
fn predict(weights: &[f64], bias: f64, features: &[f64]) -> f64 {
    let sum = weights
        .iter()
        .zip(features.iter())
        .map(|(w, x)| w * x)
        .sum::<f64>()
        + bias;

    if sum >= 0.0 {
        1.0
    } else {
        -1.0
    }
}

/// Classify a new data point using the learned weights and bias.
pub fn classify(weights: &[f64], bias: f64, features: &[f64]) -> Option<f64> {
    if weights.is_empty() || features.is_empty() {
        return None;
    }

    if weights.len() != features.len() {
        return None;
    }

    Some(predict(weights, bias, features))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_perceptron_linearly_separable() {
        let data = vec![
            (vec![1.0, 1.0], 1.0),
            (vec![2.0, 2.0], 1.0),
            (vec![3.0, 3.0], 1.0),
            (vec![-1.0, -1.0], -1.0),
            (vec![-2.0, -2.0], -1.0),
            (vec![-3.0, -3.0], -1.0),
        ];

        let result = perceptron(data, 100, 0.1);
        assert!(result.is_some());

        let (weights, bias) = result.unwrap();

        let prediction1 = classify(&weights, bias, &[2.5, 2.5]);
        assert_eq!(prediction1, Some(1.0));

        let prediction2 = classify(&weights, bias, &[-2.5, -2.5]);
        assert_eq!(prediction2, Some(-1.0));
    }

    #[test]
    fn test_perceptron_xor_like() {
        let data = vec![
            (vec![0.0, 0.0], -1.0),
            (vec![1.0, 1.0], 1.0),
            (vec![0.0, 1.0], -1.0),
            (vec![1.0, 0.0], -1.0),
        ];

        let result = perceptron(data, 100, 0.1);
        assert!(result.is_some());

        let (weights, _bias) = result.unwrap();
        assert_eq!(weights.len(), 2);
    }

    #[test]
    fn test_perceptron_single_feature() {
        let data = vec![
            (vec![1.0], 1.0),
            (vec![2.0], 1.0),
            (vec![3.0], 1.0),
            (vec![-1.0], -1.0),
            (vec![-2.0], -1.0),
            (vec![-3.0], -1.0),
        ];

        let result = perceptron(data, 100, 0.1);
        assert!(result.is_some());

        let (weights, bias) = result.unwrap();
        assert_eq!(weights.len(), 1);

        let prediction1 = classify(&weights, bias, &[5.0]);
        assert_eq!(prediction1, Some(1.0));

        let prediction2 = classify(&weights, bias, &[-5.0]);
        assert_eq!(prediction2, Some(-1.0));
    }

    #[test]
    fn test_perceptron_empty_data() {
        let result = perceptron(vec![], 100, 0.1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_perceptron_empty_features() {
        let data = vec![(vec![], 1.0), (vec![], -1.0)];
        let result = perceptron(data, 100, 0.1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_perceptron_zero_iterations() {
        let data = vec![(vec![1.0, 1.0], 1.0), (vec![-1.0, -1.0], -1.0)];

        let result = perceptron(data, 0, 0.1);
        assert!(result.is_some());

        let (weights, bias) = result.unwrap();
        assert_eq!(weights, vec![0.0, 0.0]);
        assert_eq!(bias, 0.0);
    }

    #[test]
    fn test_classify_empty_weights() {
        let result = classify(&[], 0.0, &[1.0, 2.0]);
        assert_eq!(result, None);
    }

    #[test]
    fn test_classify_empty_features() {
        let result = classify(&[1.0, 2.0], 0.0, &[]);
        assert_eq!(result, None);
    }

    #[test]
    fn test_classify_mismatched_dimensions() {
        let result = classify(&[1.0, 2.0], 0.0, &[1.0]);
        assert_eq!(result, None);
    }

    #[test]
    fn test_perceptron_different_learning_rates() {
        let data = vec![
            (vec![1.0, 1.0], 1.0),
            (vec![2.0, 2.0], 1.0),
            (vec![-1.0, -1.0], -1.0),
            (vec![-2.0, -2.0], -1.0),
        ];

        let result1 = perceptron(data.clone(), 100, 0.01);
        let result2 = perceptron(data, 100, 1.0);

        assert!(result1.is_some());
        assert!(result2.is_some());

        let (weights1, bias1) = result1.unwrap();
        let (weights2, bias2) = result2.unwrap();

        let prediction1 = classify(&weights1, bias1, &[3.0, 3.0]);
        let prediction2 = classify(&weights2, bias2, &[3.0, 3.0]);

        assert_eq!(prediction1, Some(1.0));
        assert_eq!(prediction2, Some(1.0));
    }

    #[test]
    fn test_perceptron_with_bias() {
        let data = vec![
            (vec![1.0], 1.0),
            (vec![2.0], 1.0),
            (vec![10.0], 1.0),
            (vec![0.0], -1.0),
            (vec![-1.0], -1.0),
            (vec![-10.0], -1.0),
        ];

        let result = perceptron(data, 100, 0.1);
        assert!(result.is_some());

        let (weights, bias) = result.unwrap();

        let prediction_positive = classify(&weights, bias, &[5.0]);
        let prediction_negative = classify(&weights, bias, &[-5.0]);

        assert_eq!(prediction_positive, Some(1.0));
        assert_eq!(prediction_negative, Some(-1.0));
    }
}
