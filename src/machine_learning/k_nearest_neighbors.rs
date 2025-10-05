//! K-Nearest Neighbors (KNN) algorithm implementation
//!
//! KNN is a supervised machine learning algorithm used for classification and regression.
//! It predicts the class/value of a data point based on the k nearest neighbors in the feature space.
//!
//! # Examples
//!
//! ```
//! use the_algorithms_rust::machine_learning::{DataPoint, KNearestNeighbors};
//!
//! let mut knn = KNearestNeighbors::new(3);
//!
//! let training_data = vec![
//!     DataPoint::new(vec![1.0, 1.0], "A".to_string()),
//!     DataPoint::new(vec![2.0, 2.0], "A".to_string()),
//!     DataPoint::new(vec![5.0, 5.0], "B".to_string()),
//! ];
//!
//! knn.fit(training_data);
//!
//! let prediction = knn.predict(&[1.5, 1.5]);
//! assert_eq!(prediction, Some("A".to_string()));
//! ```
use std::collections::HashMap;
/// Represents a data point with features and a label
#[derive(Debug, Clone, PartialEq)]
pub struct DataPoint {
    pub features: Vec<f64>,
    pub label: String,
}
impl DataPoint {
    /// Creates a new DataPoint
    ///
    /// # Arguments
    ///
    /// * `features` - Feature vector for the data point
    /// * `label` - Class label for the data point
    ///
    /// # Examples
    ///
    /// ```
    /// use the_algorithms_rust::machine_learning::DataPoint;
    ///
    /// let point = DataPoint::new(vec![1.0, 2.0], "A".to_string());
    /// ```
    pub fn new(features: Vec<f64>, label: String) -> Self {
        DataPoint { features, label }
    }
}
/// K-Nearest Neighbors classifier
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::machine_learning::KNearestNeighbors;
///
/// let knn = KNearestNeighbors::new(3);
/// ```
#[derive(Debug)]
pub struct KNearestNeighbors {
    k: usize,
    training_data: Vec<DataPoint>,
}
impl KNearestNeighbors {
    /// Creates a new KNN classifier with k neighbors
    ///
    /// # Arguments
    ///
    /// * `k` - Number of nearest neighbors to consider
    ///
    /// # Panics
    ///
    /// Panics if k is 0
    ///
    /// # Examples
    ///
    /// ```
    /// use the_algorithms_rust::machine_learning::KNearestNeighbors;
    ///
    /// let knn = KNearestNeighbors::new(3);
    /// ```
    pub fn new(k: usize) -> Self {
        assert!(k > 0, "k must be greater than 0");
        KNearestNeighbors {
            k,
            training_data: Vec::new(),
        }
    }
    /// Trains the KNN model with training data
    ///
    /// # Arguments
    ///
    /// * `training_data` - Vector of labeled data points
    ///
    /// # Examples
    ///
    /// ```
    /// use the_algorithms_rust::machine_learning::{DataPoint, KNearestNeighbors};
    ///
    /// let mut knn = KNearestNeighbors::new(3);
    /// let data = vec![DataPoint::new(vec![1.0, 2.0], "A".to_string())];
    /// knn.fit(data);
    /// ```
    pub fn fit(&mut self, training_data: Vec<DataPoint>) {
        self.training_data = training_data;
    }
    /// Calculates Euclidean distance between two feature vectors
    ///
    /// # Panics
    ///
    /// Panics if feature vectors have different lengths
    fn euclidean_distance(&self, a: &[f64], b: &[f64]) -> f64 {
        assert_eq!(
            a.len(),
            b.len(),
            "Feature vectors must have the same length"
        );
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
    }
    /// Predicts the label for a given data point
    ///
    /// Returns `None` if training data is empty
    ///
    /// # Arguments
    ///
    /// * `features` - Feature vector to classify
    ///
    /// # Examples
    ///
    /// ```
    /// use the_algorithms_rust::machine_learning::{DataPoint, KNearestNeighbors};
    ///
    /// let mut knn = KNearestNeighbors::new(1);
    /// knn.fit(vec![DataPoint::new(vec![1.0, 1.0], "A".to_string())]);
    /// let result = knn.predict(&[1.5, 1.5]);
    /// assert_eq!(result, Some("A".to_string()));
    /// ```
    pub fn predict(&self, features: &[f64]) -> Option<String> {
        if self.training_data.is_empty() {
            return None;
        }
        // Calculate distances to all training points
        let mut distances: Vec<(f64, &DataPoint)> = self
            .training_data
            .iter()
            .map(|point| (self.euclidean_distance(features, &point.features), point))
            .collect();
        // Sort by distance
        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
        // Take k nearest neighbors
        let k_nearest = &distances[..self.k.min(distances.len())];
        // Count votes for each label
        let mut votes: HashMap<String, usize> = HashMap::new();
        for (_, point) in k_nearest {
            *votes.entry(point.label.clone()).or_insert(0) += 1;
        }
        // Return the label with the most votes
        votes
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(label, _)| label)
    }
    /// Predicts labels for multiple data points
    ///
    /// # Arguments
    ///
    /// * `features_batch` - Slice of feature vectors to classify
    ///
    /// # Examples
    ///
    /// ```
    /// use the_algorithms_rust::machine_learning::{DataPoint, KNearestNeighbors};
    ///
    /// let mut knn = KNearestNeighbors::new(1);
    /// knn.fit(vec![DataPoint::new(vec![1.0, 1.0], "A".to_string())]);
    /// let results = knn.predict_batch(&[vec![1.5, 1.5], vec![1.2, 1.2]]);
    /// ```
    pub fn predict_batch(&self, features_batch: &[Vec<f64>]) -> Vec<Option<String>> {
        features_batch
            .iter()
            .map(|features| self.predict(features))
            .collect()
    }
    /// Calculates accuracy on test data
    ///
    /// Returns accuracy as a value between 0.0 and 1.0
    ///
    /// # Arguments
    ///
    /// * `test_data` - Test data points with known labels
    ///
    /// # Examples
    ///
    /// ```
    /// use the_algorithms_rust::machine_learning::{DataPoint, KNearestNeighbors};
    ///
    /// let mut knn = KNearestNeighbors::new(1);
    /// knn.fit(vec![DataPoint::new(vec![1.0, 1.0], "A".to_string())]);
    /// let test_data = vec![DataPoint::new(vec![1.1, 1.1], "A".to_string())];
    /// let accuracy = knn.score(&test_data);
    /// assert!(accuracy > 0.0);
    /// ```
    pub fn score(&self, test_data: &[DataPoint]) -> f64 {
        if test_data.is_empty() {
            return 0.0;
        }
        let correct = test_data
            .iter()
            .filter(|point| {
                if let Some(predicted) = self.predict(&point.features) {
                    predicted == point.label
                } else {
                    false
                }
            })
            .count();
        correct as f64 / test_data.len() as f64
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_knn_simple_classification() {
        let mut knn = KNearestNeighbors::new(3);
        let training_data = vec![
            DataPoint::new(vec![1.0, 1.0], "A".to_string()),
            DataPoint::new(vec![1.5, 1.5], "A".to_string()),
            DataPoint::new(vec![2.0, 2.0], "A".to_string()),
            DataPoint::new(vec![5.0, 5.0], "B".to_string()),
            DataPoint::new(vec![5.5, 5.5], "B".to_string()),
            DataPoint::new(vec![6.0, 6.0], "B".to_string()),
        ];
        knn.fit(training_data);
        assert_eq!(knn.predict(&[1.2, 1.2]).unwrap(), "A");
        assert_eq!(knn.predict(&[5.2, 5.2]).unwrap(), "B");
    }
    #[test]
    fn test_euclidean_distance() {
        let knn = KNearestNeighbors::new(1);
        let distance = knn.euclidean_distance(&[0.0, 0.0], &[3.0, 4.0]);
        assert!((distance - 5.0).abs() < f64::EPSILON);
    }
    #[test]
    fn test_knn_with_k_equals_one() {
        let mut knn = KNearestNeighbors::new(1);
        let training_data = vec![
            DataPoint::new(vec![1.0, 1.0], "A".to_string()),
            DataPoint::new(vec![10.0, 10.0], "B".to_string()),
        ];
        knn.fit(training_data);
        assert_eq!(knn.predict(&[1.5, 1.5]).unwrap(), "A");
        assert_eq!(knn.predict(&[9.5, 9.5]).unwrap(), "B");
    }
    #[test]
    fn test_knn_accuracy() {
        let mut knn = KNearestNeighbors::new(3);
        let training_data = vec![
            DataPoint::new(vec![1.0, 1.0], "A".to_string()),
            DataPoint::new(vec![1.5, 1.5], "A".to_string()),
            DataPoint::new(vec![2.0, 2.0], "A".to_string()),
            DataPoint::new(vec![5.0, 5.0], "B".to_string()),
            DataPoint::new(vec![5.5, 5.5], "B".to_string()),
            DataPoint::new(vec![6.0, 6.0], "B".to_string()),
        ];
        knn.fit(training_data);
        let test_data = vec![
            DataPoint::new(vec![1.2, 1.2], "A".to_string()),
            DataPoint::new(vec![5.2, 5.2], "B".to_string()),
        ];
        let accuracy = knn.score(&test_data);
        assert!((accuracy - 1.0).abs() < f64::EPSILON);
    }
    #[test]
    fn test_predict_batch() {
        let mut knn = KNearestNeighbors::new(3);
        let training_data = vec![
            DataPoint::new(vec![1.0, 1.0], "A".to_string()),
            DataPoint::new(vec![2.0, 2.0], "A".to_string()),
            DataPoint::new(vec![5.0, 5.0], "B".to_string()),
            DataPoint::new(vec![6.0, 6.0], "B".to_string()),
        ];
        knn.fit(training_data);
        let features_batch = vec![vec![1.5, 1.5], vec![5.5, 5.5]];
        let predictions = knn.predict_batch(&features_batch);
        assert_eq!(predictions[0].as_ref().unwrap(), "A");
        assert_eq!(predictions[1].as_ref().unwrap(), "B");
    }
    #[test]
    #[should_panic(expected = "k must be greater than 0")]
    fn test_knn_zero_k() {
        KNearestNeighbors::new(0);
    }
    #[test]
    fn test_empty_training_data() {
        let knn = KNearestNeighbors::new(3);
        assert!(knn.predict(&[1.0, 1.0]).is_none());
    }
    #[test]
    #[should_panic(expected = "Feature vectors must have the same length")]
    fn test_mismatched_feature_lengths() {
        let knn = KNearestNeighbors::new(1);
        knn.euclidean_distance(&[1.0, 2.0], &[1.0]);
    }
    #[test]
    fn test_predict_batch_with_empty_training() {
        let knn = KNearestNeighbors::new(3);
        let features_batch = vec![vec![1.5, 1.5], vec![5.5, 5.5]];
        let predictions = knn.predict_batch(&features_batch);
        assert!(predictions[0].is_none());
        assert!(predictions[1].is_none());
    }
    #[test]
    fn test_score_with_empty_test_data() {
        let mut knn = KNearestNeighbors::new(3);
        knn.fit(vec![DataPoint::new(vec![1.0, 1.0], "A".to_string())]);
        let accuracy = knn.score(&[]);
        assert_eq!(accuracy, 0.0);
    }
    #[test]
    fn test_k_larger_than_training_data() {
        let mut knn = KNearestNeighbors::new(10);
        let training_data = vec![
            DataPoint::new(vec![1.0, 1.0], "A".to_string()),
            DataPoint::new(vec![2.0, 2.0], "A".to_string()),
            DataPoint::new(vec![5.0, 5.0], "B".to_string()),
        ];
        knn.fit(training_data);
        // Should still work even when k > training_data.len()
        assert_eq!(knn.predict(&[1.5, 1.5]).unwrap(), "A");
    }
    #[test]
    fn test_tie_breaking() {
        let mut knn = KNearestNeighbors::new(2);
        let training_data = vec![
            DataPoint::new(vec![1.0, 1.0], "A".to_string()),
            DataPoint::new(vec![1.0, 1.0], "B".to_string()),
        ];
        knn.fit(training_data);
        // When there's a tie, it should return one of them
        let result = knn.predict(&[1.0, 1.0]);
        assert!(result.is_some());
        let prediction = result.unwrap();
        assert!(prediction == "A" || prediction == "B");
    }
}