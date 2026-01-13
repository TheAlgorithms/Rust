/// K-Nearest Neighbors (KNN) algorithm for classification.
/// KNN is a simple, instance-based learning algorithm that classifies
/// a data point based on the majority class of its k nearest neighbors.

fn euclidean_distance(p1: &[f64], p2: &[f64]) -> f64 {
    if p1.len() != p2.len() {
        return f64::INFINITY;
    }

    p1.iter()
        .zip(p2.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub fn k_nearest_neighbors(
    training_data: Vec<(Vec<f64>, f64)>,
    test_point: Vec<f64>,
    k: usize,
) -> Option<f64> {
    if training_data.is_empty() || k == 0 || k > training_data.len() {
        return None;
    }

    let mut distances: Vec<(f64, f64)> = training_data
        .iter()
        .map(|(features, label)| (euclidean_distance(&test_point, features), *label))
        .collect();

    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let k_nearest = &distances[..k];

    let mut label_counts: Vec<(f64, usize)> = Vec::new();
    for (_, label) in k_nearest {
        let found = label_counts
            .iter_mut()
            .find(|(l, _)| (l - label).abs() < 1e-10);
        if let Some((_, count)) = found {
            *count += 1;
        } else {
            label_counts.push((*label, 1));
        }
    }

    label_counts
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(label, _)| *label)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_knn() {
        let training_data = vec![
            (vec![0.0, 0.0], 0.0),
            (vec![1.0, 0.0], 0.0),
            (vec![0.0, 1.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 5.0], 1.0),
            (vec![5.0, 6.0], 1.0),
        ];

        let test_point = vec![0.5, 0.5];
        let result = k_nearest_neighbors(training_data.clone(), test_point, 3);
        assert_eq!(result, Some(0.0));

        let test_point = vec![5.5, 5.5];
        let result = k_nearest_neighbors(training_data, test_point, 3);
        assert_eq!(result, Some(1.0));
    }

    #[test]
    fn test_one_dimensional_knn() {
        let training_data = vec![
            (vec![1.0], 0.0),
            (vec![2.0], 0.0),
            (vec![3.0], 0.0),
            (vec![8.0], 1.0),
            (vec![9.0], 1.0),
            (vec![10.0], 1.0),
        ];

        let test_point = vec![2.5];
        let result = k_nearest_neighbors(training_data, test_point, 3);
        assert_eq!(result, Some(0.0));
    }

    #[test]
    fn test_knn_empty_data() {
        let training_data = vec![];
        let test_point = vec![1.0, 2.0];
        let result = k_nearest_neighbors(training_data, test_point, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_knn_invalid_k() {
        let training_data = vec![(vec![1.0], 0.0), (vec![2.0], 1.0)];
        let test_point = vec![1.5];

        // k = 0 should return None
        let result = k_nearest_neighbors(training_data.clone(), test_point.clone(), 0);
        assert_eq!(result, None);

        // k > training_data.len() should return None
        let result = k_nearest_neighbors(training_data, test_point, 10);
        assert_eq!(result, None);
    }

    #[test]
    fn test_euclidean_distance_different_dimensions() {
        let training_data = vec![
            (vec![1.0, 2.0], 0.0),
            (vec![2.0, 3.0], 0.0),
            (vec![5.0], 1.0),
        ];
        let test_point = vec![1.5, 2.5];
        let result = k_nearest_neighbors(training_data, test_point, 2);
        assert_eq!(result, Some(0.0));
    }
}
