use super::optimization::gradient_descent;
use std::f64::consts::E;

/// Returns the weights after performing Logistic regression on the input data points.
pub fn logistic_regression(
    data_points: Vec<(Vec<f64>, f64)>,
    iterations: usize,
    learning_rate: f64,
) -> Option<Vec<f64>> {
    if data_points.is_empty() {
        return None;
    }

    let num_features = data_points[0].0.len() + 1;
    let mut params = vec![0.0; num_features];

    let derivative_fn = |params: &[f64]| derivative(params, &data_points);

    gradient_descent(derivative_fn, &mut params, learning_rate, iterations as i32);

    Some(params)
}

fn derivative(params: &[f64], data_points: &[(Vec<f64>, f64)]) -> Vec<f64> {
    let num_features = params.len();
    let mut gradients = vec![0.0; num_features];

    for (features, y_i) in data_points {
        let z = params[0]
            + params[1..]
                .iter()
                .zip(features)
                .map(|(p, x)| p * x)
                .sum::<f64>();
        let prediction = 1.0 / (1.0 + E.powf(-z));

        gradients[0] += prediction - y_i;
        for (i, x_i) in features.iter().enumerate() {
            gradients[i + 1] += (prediction - y_i) * x_i;
        }
    }

    gradients
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_logistic_regression_simple() {
        let data = vec![
            (vec![0.0], 0.0),
            (vec![1.0], 0.0),
            (vec![2.0], 0.0),
            (vec![3.0], 1.0),
            (vec![4.0], 1.0),
            (vec![5.0], 1.0),
        ];

        let result = logistic_regression(data, 10000, 0.05);
        assert!(result.is_some());

        let params = result.unwrap();
        assert!((params[0] + 17.65).abs() < 1.0);
        assert!((params[1] - 7.13).abs() < 1.0);
    }

    #[test]
    fn test_logistic_regression_extreme_data() {
        let data = vec![
            (vec![-100.0], 0.0),
            (vec![-10.0], 0.0),
            (vec![0.0], 0.0),
            (vec![10.0], 1.0),
            (vec![100.0], 1.0),
        ];

        let result = logistic_regression(data, 10000, 0.05);
        assert!(result.is_some());

        let params = result.unwrap();
        assert!((params[0] + 6.20).abs() < 1.0);
        assert!((params[1] - 5.5).abs() < 1.0);
    }

    #[test]
    fn test_logistic_regression_no_data() {
        let result = logistic_regression(vec![], 5000, 0.1);
        assert_eq!(result, None);
    }
}
