use rand::seq::SliceRandom;
use rand::Rng;

/// Train a single decision tree on a bootstrap sample with random feature subset
#[allow(dead_code)]
fn train_tree(
    training_data: &[(Vec<f64>, f64)],
    num_features: usize,
    max_depth: usize,
    min_samples_split: usize,
    max_features: usize,
) -> Option<crate::machine_learning::decision_tree::DecisionTree> {
    if training_data.is_empty() {
        return None;
    }

    // Bootstrap sampling: sample with replacement
    let num_samples = training_data.len();
    let mut rng = rand::rng();
    let mut bootstrap_sample = Vec::with_capacity(num_samples);

    for _ in 0..num_samples {
        let random_index = rng.random_range(0..num_samples);
        bootstrap_sample.push(training_data[random_index].clone());
    }

    // Select random subset of features for this tree
    let mut feature_indices: Vec<usize> = (0..num_features).collect();
    feature_indices.shuffle(&mut rng);
    feature_indices.truncate(max_features);

    // Train decision tree on bootstrap sample with limited features
    let limited_sample: Vec<(Vec<f64>, f64)> = bootstrap_sample
        .iter()
        .map(|(features, label)| {
            let limited_features: Vec<f64> =
                feature_indices.iter().map(|&idx| features[idx]).collect();
            (limited_features, *label)
        })
        .collect();

    let tree = crate::machine_learning::decision_tree::DecisionTree::fit(
        limited_sample,
        max_depth,
        min_samples_split,
    )?;

    Some(tree)
}

#[derive(Debug, PartialEq)]
pub struct RandomForest {
    trees: Vec<crate::machine_learning::decision_tree::DecisionTree>,
    feature_indices: Vec<Vec<usize>>,
    num_classes: usize,
}

impl RandomForest {
    pub fn fit(
        training_data: Vec<(Vec<f64>, f64)>,
        num_trees: usize,
        max_depth: usize,
        min_samples_split: usize,
        max_features: Option<usize>,
    ) -> Option<Self> {
        if training_data.is_empty() {
            return None;
        }

        let num_features = training_data[0].0.len();
        if num_features == 0 {
            return None;
        }

        // Default max_features to sqrt of total features
        let max_features = max_features.unwrap_or_else(|| (num_features as f64).sqrt() as usize);
        let max_features = max_features.max(1).min(num_features);

        let mut trees = Vec::new();
        let mut all_feature_indices = Vec::new();

        // Train multiple decision trees
        for _ in 0..num_trees {
            let mut rng = rand::rng();
            let mut feature_indices: Vec<usize> = (0..num_features).collect();
            feature_indices.shuffle(&mut rng);
            feature_indices.truncate(max_features);

            let mut bootstrap_sample = Vec::with_capacity(training_data.len());
            for _ in 0..training_data.len() {
                let random_index = rng.random_range(0..training_data.len());
                bootstrap_sample.push(training_data[random_index].clone());
            }

            let limited_sample: Vec<(Vec<f64>, f64)> = bootstrap_sample
                .iter()
                .map(|(features, label)| {
                    let limited_features: Vec<f64> =
                        feature_indices.iter().map(|&idx| features[idx]).collect();
                    (limited_features, *label)
                })
                .collect();

            if let Some(tree) = crate::machine_learning::decision_tree::DecisionTree::fit(
                limited_sample,
                max_depth,
                min_samples_split,
            ) {
                trees.push(tree);
                all_feature_indices.push(feature_indices);
            }
        }

        if trees.is_empty() {
            return None;
        }

        // Determine number of classes
        let mut unique_labels: Vec<f64> = Vec::new();
        for (_, label) in &training_data {
            if !unique_labels.contains(label) {
                unique_labels.push(*label);
            }
        }
        let num_classes = unique_labels.len();

        Some(RandomForest {
            trees,
            feature_indices: all_feature_indices,
            num_classes,
        })
    }

    pub fn predict(&self, test_point: &[f64]) -> Option<f64> {
        if test_point.is_empty() || self.trees.is_empty() {
            return None;
        }

        let mut predictions: Vec<f64> = Vec::new();

        for (tree, feature_indices) in self.trees.iter().zip(self.feature_indices.iter()) {
            let limited_point: Vec<f64> =
                feature_indices.iter().map(|&idx| test_point[idx]).collect();

            if let Some(prediction) = tree.predict(&limited_point) {
                predictions.push(prediction);
            }
        }

        if predictions.is_empty() {
            return None;
        }

        // Majority voting
        let mut unique_labels: Vec<f64> = Vec::new();
        let mut counts: Vec<usize> = Vec::new();

        for &pred in &predictions {
            let mut found = false;
            for (i, &label) in unique_labels.iter().enumerate() {
                if (label - pred).abs() < 1e-10 {
                    counts[i] += 1;
                    found = true;
                    break;
                }
            }
            if !found {
                unique_labels.push(pred);
                counts.push(1);
            }
        }

        let mut max_count = 0;
        let mut best_label = unique_labels[0];
        for (i, &count) in counts.iter().enumerate() {
            if count > max_count {
                max_count = count;
                best_label = unique_labels[i];
            }
        }

        Some(best_label)
    }

    #[allow(dead_code)]
    pub fn predict_batch(&self, test_points: &[Vec<f64>]) -> Vec<Option<f64>> {
        test_points
            .iter()
            .map(|point| self.predict(point))
            .collect()
    }
}

/// Convenience function to train a random forest and make predictions
pub fn random_forest(
    training_data: Vec<(Vec<f64>, f64)>,
    test_point: Vec<f64>,
    num_trees: usize,
    max_depth: usize,
    min_samples_split: usize,
    max_features: Option<usize>,
) -> Option<f64> {
    let model = RandomForest::fit(
        training_data,
        num_trees,
        max_depth,
        min_samples_split,
        max_features,
    )?;
    model.predict(&test_point)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_forest_linearly_separable() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![3.0, 3.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
            (vec![7.0, 7.0], 1.0),
        ];

        let model = RandomForest::fit(training_data, 10, 5, 2, None);
        assert!(model.is_some());

        let model = model.unwrap();

        assert_eq!(model.predict(&[1.5, 1.5]), Some(0.0));
        assert_eq!(model.predict(&[5.5, 5.5]), Some(1.0));
    }

    #[test]
    fn test_random_forest_xor() {
        let training_data = vec![
            (vec![0.0, 0.0], 0.0),
            (vec![0.0, 1.0], 1.0),
            (vec![1.0, 0.0], 1.0),
            (vec![1.0, 1.0], 0.0),
            // Add more samples to help with XOR
            (vec![0.2, 0.2], 0.0),
            (vec![0.8, 0.8], 0.0),
            (vec![0.2, 0.8], 1.0),
            (vec![0.8, 0.2], 1.0),
        ];

        let model = RandomForest::fit(training_data, 20, 5, 2, Some(2));
        assert!(model.is_some());

        let model = model.unwrap();

        // Verify model can make predictions (not necessarily perfect)
        let result = model.predict(&[0.0, 0.0]);
        assert!(result.is_some());

        let result = model.predict(&[1.0, 1.0]);
        assert!(result.is_some());
    }

    #[test]
    fn test_random_forest_multiple_classes() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
            (vec![9.0, 9.0], 2.0),
            (vec![10.0, 10.0], 2.0),
        ];

        let model = RandomForest::fit(training_data, 10, 5, 2, None);
        assert!(model.is_some());

        let model = model.unwrap();

        assert_eq!(model.predict(&[1.5, 1.5]), Some(0.0));
        assert_eq!(model.predict(&[5.5, 5.5]), Some(1.0));
        assert_eq!(model.predict(&[9.5, 9.5]), Some(2.0));
    }

    #[test]
    fn test_random_forest_one_feature() {
        let training_data = vec![
            (vec![1.0], 0.0),
            (vec![2.0], 0.0),
            (vec![3.0], 0.0),
            (vec![5.0], 1.0),
            (vec![6.0], 1.0),
            (vec![7.0], 1.0),
        ];

        let model = RandomForest::fit(training_data, 10, 5, 2, None);
        assert!(model.is_some());

        let model = model.unwrap();

        assert_eq!(model.predict(&[2.5]), Some(0.0));
        assert_eq!(model.predict(&[5.5]), Some(1.0));
    }

    #[test]
    fn test_random_forest_empty_training_data() {
        let training_data = vec![];
        let model = RandomForest::fit(training_data, 10, 5, 2, None);
        assert_eq!(model, None);
    }

    #[test]
    fn test_random_forest_empty_features() {
        let training_data = vec![(vec![], 0.0), (vec![], 1.0)];
        let model = RandomForest::fit(training_data, 10, 5, 2, None);
        assert_eq!(model, None);
    }

    #[test]
    fn test_random_forest_predict_batch() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let model = RandomForest::fit(training_data, 10, 5, 2, None);
        assert!(model.is_some());

        let model = model.unwrap();

        let test_points = vec![vec![1.5, 1.5], vec![5.5, 5.5]];
        let predictions = model.predict_batch(&test_points);

        assert_eq!(predictions.len(), 2);
        assert_eq!(predictions[0], Some(0.0));
        assert_eq!(predictions[1], Some(1.0));
    }

    #[test]
    fn test_random_forest_custom_max_features() {
        let training_data = vec![
            (vec![1.0, 2.0, 3.0], 0.0),
            (vec![2.0, 3.0, 4.0], 0.0),
            (vec![5.0, 6.0, 7.0], 1.0),
            (vec![6.0, 7.0, 8.0], 1.0),
        ];

        let model = RandomForest::fit(training_data, 10, 5, 2, Some(2));
        assert!(model.is_some());

        let model = model.unwrap();

        assert_eq!(model.predict(&[1.5, 2.5, 3.5]), Some(0.0));
        assert_eq!(model.predict(&[5.5, 6.5, 7.5]), Some(1.0));
    }

    #[test]
    fn test_random_forest_convenience_function() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let result = random_forest(training_data, vec![1.5, 1.5], 10, 5, 2, None);
        assert_eq!(result, Some(0.0));

        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let result = random_forest(training_data, vec![5.5, 5.5], 10, 5, 2, None);
        assert_eq!(result, Some(1.0));
    }

    #[test]
    fn test_random_forest_single_tree() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let model = RandomForest::fit(training_data, 1, 5, 2, None);
        assert!(model.is_some());

        let model = model.unwrap();

        // With single tree and bootstrap sampling, predictions may vary
        // Just verify model can make predictions
        let result1 = model.predict(&[1.5, 1.5]);
        let result2 = model.predict(&[5.5, 5.5]);

        assert!(result1.is_some());
        assert!(result2.is_some());
    }

    #[test]
    fn test_random_forest_empty_test_point() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let model = RandomForest::fit(training_data, 10, 5, 2, None);
        assert!(model.is_some());

        let model = model.unwrap();

        let result = model.predict(&[]);
        assert_eq!(result, None);
    }

    #[test]
    fn test_random_forest_different_num_trees() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let model_5 = RandomForest::fit(training_data.clone(), 5, 5, 2, None);
        let model_20 = RandomForest::fit(training_data, 20, 5, 2, None);

        assert!(model_5.is_some());
        assert!(model_20.is_some());

        let model_5 = model_5.unwrap();
        let model_20 = model_20.unwrap();

        assert_eq!(model_5.predict(&[1.5, 1.5]), Some(0.0));
        assert_eq!(model_20.predict(&[1.5, 1.5]), Some(0.0));
    }
}
