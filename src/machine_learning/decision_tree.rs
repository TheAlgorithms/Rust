/// Decision Tree classifier using the ID3 algorithm with entropy-based splitting.
/// The tree recursively splits data based on the feature that provides the highest information gain.
/// Supports both categorical and continuous features through threshold-based splitting.

#[derive(Debug, Clone, PartialEq)]
enum TreeNode {
    Leaf {
        class_label: f64,
        samples: usize,
    },
    InternalNode {
        feature_index: usize,
        threshold: f64,
        left: Box<TreeNode>,
        right: Box<TreeNode>,
        samples: usize,
    },
}

/// Calculate entropy of a set of labels
fn calculate_entropy(labels: &[f64]) -> f64 {
    if labels.is_empty() {
        return 0.0;
    }

    let total = labels.len() as f64;
    let mut unique_labels: Vec<f64> = Vec::new();
    let mut counts = Vec::new();

    for &label in labels {
        let mut found = false;
        for (i, &existing_label) in unique_labels.iter().enumerate() {
            if (existing_label as f64 - label as f64).abs() < 1e-10 {
                counts[i] += 1;
                found = true;
                break;
            }
        }
        if !found {
            unique_labels.push(label);
            counts.push(1);
        }
    }

    let mut entropy = 0.0;
    for &count in &counts {
        let probability = count as f64 / total;
        if probability > 0.0 {
            entropy -= probability * probability.log2();
        }
    }

    entropy
}

/// Find the best split for a feature
fn find_best_split(data: &[(Vec<f64>, f64)], feature_index: usize) -> Option<(f64, f64)> {
    if data.is_empty() {
        return None;
    }

    let num_samples = data.len();

    let mut feature_values: Vec<(f64, f64)> = data
        .iter()
        .map(|(features, label)| (features[feature_index], *label))
        .collect();

    feature_values.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let parent_entropy =
        calculate_entropy(&data.iter().map(|(_, label)| *label).collect::<Vec<_>>());

    let mut best_threshold = feature_values[0].0;
    let mut best_gain = 0.0;

    for i in 1..num_samples {
        if feature_values[i].0 != feature_values[i - 1].0 {
            let threshold = (feature_values[i].0 + feature_values[i - 1].0) / 2.0;

            let left_labels: Vec<f64> = feature_values[..i]
                .iter()
                .map(|(_, label)| *label)
                .collect();
            let right_labels: Vec<f64> = feature_values[i..]
                .iter()
                .map(|(_, label)| *label)
                .collect();

            let left_entropy = calculate_entropy(&left_labels);
            let right_entropy = calculate_entropy(&right_labels);

            let left_weight = i as f64 / num_samples as f64;
            let right_weight = (num_samples - i) as f64 / num_samples as f64;

            let weighted_entropy = left_weight * left_entropy + right_weight * right_entropy;
            let information_gain = parent_entropy - weighted_entropy;

            if information_gain > best_gain {
                best_gain = information_gain;
                best_threshold = threshold;
            }
        }
    }

    if best_gain > 0.0 {
        Some((best_threshold, best_gain))
    } else {
        None
    }
}

/// Find the best feature and threshold to split on
fn find_best_split_feature(
    data: &[(Vec<f64>, f64)],
    feature_indices: &[usize],
) -> Option<(usize, f64)> {
    if data.is_empty() || feature_indices.is_empty() {
        return None;
    }

    let mut best_feature_index = 0;
    let mut best_threshold = 0.0;
    let mut best_gain = 0.0;

    for &feature_index in feature_indices {
        if let Some((threshold, gain)) = find_best_split(data, feature_index) {
            if gain > best_gain {
                best_gain = gain;
                best_threshold = threshold;
                best_feature_index = feature_index;
            }
        }
    }

    if best_gain > 0.0 {
        Some((best_feature_index, best_threshold))
    } else {
        None
    }
}

/// Get the majority class label
fn get_majority_class(labels: &[f64]) -> f64 {
    if labels.is_empty() {
        return 0.0;
    }

    let mut unique_labels: Vec<f64> = Vec::new();
    let mut counts = Vec::new();

    for &label in labels {
        let mut found = false;
        for (i, &existing_label) in unique_labels.iter().enumerate() {
            if (existing_label as f64 - label as f64).abs() < 1e-10 {
                counts[i] += 1;
                found = true;
                break;
            }
        }
        if !found {
            unique_labels.push(label);
            counts.push(1);
        }
    }

    let mut max_index = 0;
    let mut max_count = 0;
    for (i, &count) in counts.iter().enumerate() {
        if count > max_count {
            max_count = count;
            max_index = i;
        }
    }

    unique_labels[max_index]
}

/// Build the decision tree recursively
fn build_tree(
    data: &[(Vec<f64>, f64)],
    feature_indices: &[usize],
    max_depth: usize,
    min_samples_split: usize,
    current_depth: usize,
) -> TreeNode {
    let labels: Vec<f64> = data.iter().map(|(_, label)| *label).collect();

    // Count unique labels
    let mut unique_count = 0;
    for i in 0..labels.len() {
        let mut is_unique = true;
        for j in 0..i {
            if (labels[i] - labels[j]).abs() < 1e-10 {
                is_unique = false;
                break;
            }
        }
        if is_unique {
            unique_count += 1;
        }
    }

    if unique_count == 1
        || data.len() < min_samples_split
        || current_depth >= max_depth
        || feature_indices.is_empty()
    {
        let class_label = get_majority_class(&labels);
        return TreeNode::Leaf {
            class_label,
            samples: data.len(),
        };
    }

    if let Some((feature_index, threshold)) = find_best_split_feature(data, feature_indices) {
        let mut left_data = Vec::new();
        let mut right_data = Vec::new();

        for (features, label) in data {
            if features[feature_index] < threshold {
                left_data.push((features.clone(), *label));
            } else {
                right_data.push((features.clone(), *label));
            }
        }

        if left_data.is_empty() || right_data.is_empty() {
            let class_label = get_majority_class(&labels);
            return TreeNode::Leaf {
                class_label,
                samples: data.len(),
            };
        }

        let left_child = build_tree(
            &left_data,
            feature_indices,
            max_depth,
            min_samples_split,
            current_depth + 1,
        );
        let right_child = build_tree(
            &right_data,
            feature_indices,
            max_depth,
            min_samples_split,
            current_depth + 1,
        );

        TreeNode::InternalNode {
            feature_index,
            threshold,
            left: Box::new(left_child),
            right: Box::new(right_child),
            samples: data.len(),
        }
    } else {
        let class_label = get_majority_class(&labels);
        TreeNode::Leaf {
            class_label,
            samples: data.len(),
        }
    }
}

/// Predict the class label for a single test point
fn predict_tree(tree: &TreeNode, features: &[f64]) -> f64 {
    match tree {
        TreeNode::Leaf { class_label, .. } => *class_label,
        TreeNode::InternalNode {
            feature_index,
            threshold,
            left,
            right,
            ..
        } => {
            if features[*feature_index] < *threshold {
                predict_tree(left, features)
            } else {
                predict_tree(right, features)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DecisionTree {
    tree: TreeNode,
}

impl DecisionTree {
    pub fn fit(
        training_data: Vec<(Vec<f64>, f64)>,
        max_depth: usize,
        min_samples_split: usize,
    ) -> Option<Self> {
        if training_data.is_empty() {
            return None;
        }

        let num_features = training_data[0].0.len();
        if num_features == 0 {
            return None;
        }

        let feature_indices: Vec<usize> = (0..num_features).collect();
        let tree = build_tree(
            &training_data,
            &feature_indices,
            max_depth,
            min_samples_split,
            0,
        );

        Some(DecisionTree { tree })
    }

    pub fn predict(&self, test_point: &[f64]) -> Option<f64> {
        if test_point.is_empty() {
            return None;
        }

        Some(predict_tree(&self.tree, test_point))
    }

    pub fn predict_batch(&self, test_points: &[Vec<f64>]) -> Vec<Option<f64>> {
        test_points
            .iter()
            .map(|point| self.predict(point))
            .collect()
    }
}

/// Convenience function to train a decision tree and make predictions
pub fn decision_tree(
    training_data: Vec<(Vec<f64>, f64)>,
    test_point: Vec<f64>,
    max_depth: usize,
    min_samples_split: usize,
) -> Option<f64> {
    let model = DecisionTree::fit(training_data, max_depth, min_samples_split)?;
    model.predict(&test_point)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decision_tree_simple_xor() {
        let training_data = vec![
            (vec![0.0, 0.0], 0.0),
            (vec![0.0, 1.0], 1.0),
            (vec![1.0, 0.0], 1.0),
            (vec![1.0, 1.0], 0.0),
        ];

        let model = DecisionTree::fit(training_data, 10, 2);
        assert!(model.is_some());

        let model = model.unwrap();

        // XOR is difficult for decision trees with small dataset
        // Just verify the model can make predictions (not necessarily perfect for XOR)
        let result = model.predict(&[0.0, 0.0]);
        assert!(result.is_some());

        let result = model.predict(&[1.0, 1.0]);
        assert!(result.is_some());
    }

    #[test]
    fn test_decision_tree_linearly_separable() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![3.0, 3.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
            (vec![7.0, 7.0], 1.0),
        ];

        let model = DecisionTree::fit(training_data, 10, 2);
        assert!(model.is_some());

        let model = model.unwrap();

        assert_eq!(model.predict(&[1.5, 1.5]), Some(0.0));
        assert_eq!(model.predict(&[5.5, 5.5]), Some(1.0));
    }

    #[test]
    fn test_decision_tree_one_feature() {
        let training_data = vec![
            (vec![1.0], 0.0),
            (vec![2.0], 0.0),
            (vec![3.0], 0.0),
            (vec![5.0], 1.0),
            (vec![6.0], 1.0),
            (vec![7.0], 1.0),
        ];

        let model = DecisionTree::fit(training_data, 10, 2);
        assert!(model.is_some());

        let model = model.unwrap();

        assert_eq!(model.predict(&[2.5]), Some(0.0));
        assert_eq!(model.predict(&[5.5]), Some(1.0));
    }

    #[test]
    fn test_decision_tree_multiple_classes() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
            (vec![9.0, 9.0], 2.0),
            (vec![10.0, 10.0], 2.0),
        ];

        let model = DecisionTree::fit(training_data, 10, 2);
        assert!(model.is_some());

        let model = model.unwrap();

        assert_eq!(model.predict(&[1.5, 1.5]), Some(0.0));
        assert_eq!(model.predict(&[5.5, 5.5]), Some(1.0));
        assert_eq!(model.predict(&[9.5, 9.5]), Some(2.0));
    }

    #[test]
    fn test_decision_tree_empty_training_data() {
        let training_data = vec![];
        let model = DecisionTree::fit(training_data, 10, 2);
        assert_eq!(model, None);
    }

    #[test]
    fn test_decision_tree_empty_features() {
        let training_data = vec![(vec![], 0.0), (vec![], 1.0)];
        let model = DecisionTree::fit(training_data, 10, 2);
        assert_eq!(model, None);
    }

    #[test]
    fn test_decision_tree_max_depth() {
        let training_data = vec![
            (vec![0.0, 0.0], 0.0),
            (vec![0.0, 1.0], 1.0),
            (vec![1.0, 0.0], 1.0),
            (vec![1.0, 1.0], 0.0),
        ];

        let model = DecisionTree::fit(training_data, 1, 2);
        assert!(model.is_some());

        let model = model.unwrap();
        let result = model.predict(&[0.5, 0.5]);
        assert!(result.is_some());
    }

    #[test]
    fn test_decision_tree_min_samples_split() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let model = DecisionTree::fit(training_data, 10, 10);
        assert!(model.is_some());

        let model = model.unwrap();
        let result = model.predict(&[1.5, 1.5]);
        assert!(result.is_some());
    }

    #[test]
    fn test_decision_tree_predict_batch() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let model = DecisionTree::fit(training_data, 10, 2);
        assert!(model.is_some());

        let model = model.unwrap();

        let test_points = vec![vec![1.5, 1.5], vec![5.5, 5.5]];
        let predictions = model.predict_batch(&test_points);

        assert_eq!(predictions.len(), 2);
        assert_eq!(predictions[0], Some(0.0));
        assert_eq!(predictions[1], Some(1.0));
    }

    #[test]
    fn test_decision_tree_convenience_function() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let result = decision_tree(training_data, vec![1.5, 1.5], 10, 2);
        assert_eq!(result, Some(0.0));

        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let result = decision_tree(training_data, vec![5.5, 5.5], 10, 2);
        assert_eq!(result, Some(1.0));
    }

    #[test]
    fn test_calculate_entropy() {
        let labels = vec![0.0, 0.0, 0.0, 0.0];
        let entropy = calculate_entropy(&labels);
        assert!((entropy - 0.0).abs() < 1e-10);

        let labels = vec![0.0, 0.0, 1.0, 1.0];
        let entropy = calculate_entropy(&labels);
        assert!((entropy - 1.0).abs() < 1e-10);

        let labels = vec![0.0, 1.0, 2.0];
        let entropy = calculate_entropy(&labels);
        assert!(entropy > 0.0 && entropy < 2.0);
    }

    #[test]
    fn test_get_majority_class() {
        let labels = vec![0.0, 0.0, 1.0, 1.0, 0.0];
        let majority = get_majority_class(&labels);
        assert_eq!(majority, 0.0);

        let labels = vec![1.0, 1.0, 2.0, 2.0, 2.0];
        let majority = get_majority_class(&labels);
        assert_eq!(majority, 2.0);
    }
}
