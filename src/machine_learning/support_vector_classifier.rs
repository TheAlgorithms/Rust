//! Support Vector Classifier (SVC)
//!
//! This module implements a Support Vector Machine classifier with support for
//! linear and RBF (Radial Basis Function) kernels. It uses the dual formulation
//! of the SVM optimization problem.
//!
//! # Example
//! ```
//! use ndarray::array;
//! use the_algorithms_rust::machine_learning::{SVC, Kernel};
//!
//! let observations = vec![
//!     array![0.0, 1.0],
//!     array![0.0, 2.0],
//!     array![1.0, 1.0],
//!     array![1.0, 2.0],
//! ];
//! let classes = array![1.0, 1.0, -1.0, -1.0];
//!
//! let mut svc = SVC::new(Kernel::Linear, f64::INFINITY).unwrap();
//! svc.fit(&observations, &classes).unwrap();
//! assert_eq!(svc.predict(&array![0.0, 1.0]), 1.0);
//! assert_eq!(svc.predict(&array![1.0, 1.0]), -1.0);
//! ```

use ndarray::{Array1, Array2};
use std::f64;

/// Kernel types supported by the SVC
#[derive(Debug, Clone)]
pub enum Kernel {
    /// Linear kernel: K(x, y) = x · y
    Linear,
    /// RBF kernel: K(x, y) = exp(-gamma * ||x - y||²)
    Rbf { gamma: f64 },
}

/// Support Vector Classifier
///
/// A binary classifier that finds the optimal hyperplane to separate two classes.
/// Uses the dual formulation with support for different kernel functions.
#[derive(Debug)]
pub struct SVC {
    kernel: Kernel,
    regularization: f64,
    observations: Vec<Array1<f64>>,
    classes: Array1<f64>,
    optimum: Array1<f64>,
    offset: f64,
}

/// Errors that can occur when creating or using an SVC
#[derive(Debug, PartialEq)]
pub enum SVCError {
    InvalidGamma,
    InvalidRegularization,
    EmptyData,
    MismatchedDimensions,
}

impl std::fmt::Display for SVCError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SVCError::InvalidGamma => write!(f, "gamma must be > 0"),
            SVCError::InvalidRegularization => write!(f, "regularization must be > 0"),
            SVCError::EmptyData => write!(f, "observations and classes cannot be empty"),
            SVCError::MismatchedDimensions => {
                write!(f, "number of observations must match number of classes")
            }
        }
    }
}

impl std::error::Error for SVCError {}

impl SVC {
    /// Creates a new Support Vector Classifier
    ///
    /// # Arguments
    /// * `kernel` - The kernel function to use
    /// * `regularization` - Soft margin constraint (C parameter), use `f64::INFINITY` for hard margin
    ///
    /// # Errors
    /// Returns an error if gamma (for RBF kernel) or regularization are invalid
    ///
    /// # Example
    /// ```
    /// use the_algorithms_rust::machine_learning::{SVC, Kernel};
    ///
    /// let svc = SVC::new(Kernel::Linear, f64::INFINITY).unwrap();
    /// let svc_rbf = SVC::new(Kernel::Rbf { gamma: 0.5 }, 1.0).unwrap();
    /// ```
    pub fn new(kernel: Kernel, regularization: f64) -> Result<Self, SVCError> {
        if regularization <= 0.0 {
            return Err(SVCError::InvalidRegularization);
        }

        if let Kernel::Rbf { gamma } = kernel {
            if gamma <= 0.0 {
                return Err(SVCError::InvalidGamma);
            }
        }

        Ok(SVC {
            kernel,
            regularization,
            observations: Vec::new(),
            classes: Array1::zeros(0),
            optimum: Array1::zeros(0),
            offset: 0.0,
        })
    }

    /// Computes the kernel function between two vectors
    fn kernel_function(&self, v1: &Array1<f64>, v2: &Array1<f64>) -> f64 {
        match &self.kernel {
            Kernel::Linear => v1.dot(v2),
            Kernel::Rbf { gamma } => {
                let diff = v1 - v2;
                let norm_sq = diff.dot(&diff);
                (-gamma * norm_sq).exp()
            }
        }
    }

    /// Fits the SVC with training data
    ///
    /// # Arguments
    /// * `observations` - Training feature vectors
    /// * `classes` - Class labels (should be 1.0 or -1.0)
    ///
    /// # Errors
    /// Returns an error if data is empty or dimensions don't match
    ///
    /// # Example
    /// ```
    /// use ndarray::array;
    /// use the_algorithms_rust::machine_learning::{SVC, Kernel};
    ///
    /// let observations = vec![array![0.0, 1.0], array![1.0, 0.0]];
    /// let classes = array![1.0, -1.0];
    /// let mut svc = SVC::new(Kernel::Linear, f64::INFINITY).unwrap();
    /// svc.fit(&observations, &classes).unwrap();
    /// ```
    pub fn fit(
        &mut self,
        observations: &[Array1<f64>],
        classes: &Array1<f64>,
    ) -> Result<(), SVCError> {
        if observations.is_empty() || classes.is_empty() {
            return Err(SVCError::EmptyData);
        }

        if observations.len() != classes.len() {
            return Err(SVCError::MismatchedDimensions);
        }

        self.observations = observations.to_vec();
        self.classes.clone_from(classes);

        let n = classes.len();

        // Solve the dual optimization problem
        // We use a simple gradient descent approach for educational purposes
        // In production, you'd want to use a proper QP solver
        let optimum = self.solve_dual(n);
        self.optimum = optimum;

        // Calculate offset (bias term)
        self.offset = self.calculate_offset(n);

        Ok(())
    }

    /// Solves the dual optimization problem using a simple gradient descent
    ///
    /// This is a simplified solver for educational purposes.
    /// In production, use a proper QP solver like OSQP or similar.
    fn solve_dual(&self, n: usize) -> Array1<f64> {
        let mut lambda = Array1::from_elem(n, 0.5);
        let learning_rate = 0.1;
        let iterations = 5000;
        let tolerance = 1e-8;

        // Precompute kernel matrix for efficiency
        let mut kernel_matrix = Array2::zeros((n, n));
        for i in 0..n {
            for j in 0..n {
                kernel_matrix[[i, j]] =
                    self.kernel_function(&self.observations[i], &self.observations[j]);
            }
        }

        for iter in 0..iterations {
            let mut gradient = Array1::zeros(n);

            // Compute gradient of the dual objective
            for i in 0..n {
                let mut sum = 0.0;
                for j in 0..n {
                    sum += lambda[j] * self.classes[j] * kernel_matrix[[i, j]];
                }
                gradient[i] = self.classes[i] * sum - 1.0;
            }

            // Update lambda with gradient descent
            let old_lambda = lambda.clone();

            // Adaptive learning rate
            let lr = learning_rate / (1.0 + iter as f64 / 1000.0);
            lambda = &lambda - lr * &gradient;

            // Project onto constraints: 0 <= lambda <= C
            for i in 0..n {
                lambda[i] = lambda[i].max(0.0).min(self.regularization);
            }

            // Enforce sum(lambda * y) = 0 constraint using projection
            let mut sum_ly = 0.0;
            for i in 0..n {
                sum_ly += lambda[i] * self.classes[i];
            }

            // Better constraint enforcement
            let mut correction = sum_ly / n as f64;
            for _ in 0..10 {
                for i in 0..n {
                    let delta = correction * self.classes[i];
                    let new_val = lambda[i] - delta;
                    lambda[i] = new_val.max(0.0).min(self.regularization);
                }

                // Recalculate sum
                sum_ly = 0.0;
                for i in 0..n {
                    sum_ly += lambda[i] * self.classes[i];
                }
                correction = sum_ly / n as f64;

                if sum_ly.abs() < 1e-10 {
                    break;
                }
            }

            // Check convergence
            let diff = &lambda - &old_lambda;
            if diff.dot(&diff).sqrt() < tolerance {
                break;
            }
        }

        lambda
    }

    /// Calculates the offset (bias) term
    fn calculate_offset(&self, n: usize) -> f64 {
        let mut sum = 0.0;
        let mut count = 0;

        // Calculate bias using support vectors (lambda > threshold and < C)
        let threshold = 1e-5;
        for i in 0..n {
            if self.optimum[i] > threshold && self.optimum[i] < self.regularization - threshold {
                let mut kernel_sum = 0.0;
                for j in 0..n {
                    kernel_sum += self.optimum[j]
                        * self.classes[j]
                        * self.kernel_function(&self.observations[j], &self.observations[i]);
                }
                sum += self.classes[i] - kernel_sum;
                count += 1;
            }
        }

        // If no clear support vectors, use all points
        if count == 0 {
            for i in 0..n {
                let mut kernel_sum = 0.0;
                for j in 0..n {
                    kernel_sum += self.optimum[j]
                        * self.classes[j]
                        * self.kernel_function(&self.observations[j], &self.observations[i]);
                }
                sum += self.classes[i] - kernel_sum;
            }
            sum / n as f64
        } else {
            sum / count as f64
        }
    }

    /// Predicts the class of a new observation
    ///
    /// # Arguments
    /// * `observation` - Feature vector to classify
    ///
    /// # Returns
    /// The predicted class (1.0 or -1.0)
    ///
    /// # Example
    /// ```
    /// use ndarray::array;
    /// use the_algorithms_rust::machine_learning::{SVC, Kernel};
    ///
    /// let observations = vec![array![0.0, 1.0], array![1.0, 0.0]];
    /// let classes = array![1.0, -1.0];
    /// let mut svc = SVC::new(Kernel::Linear, f64::INFINITY).unwrap();
    /// svc.fit(&observations, &classes).unwrap();
    ///
    /// let prediction = svc.predict(&array![0.5, 0.5]);
    /// assert!(prediction == 1.0 || prediction == -1.0);
    /// ```
    pub fn predict(&self, observation: &Array1<f64>) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.classes.len() {
            sum += self.optimum[i]
                * self.classes[i]
                * self.kernel_function(&self.observations[i], observation);
        }

        if sum + self.offset >= 0.0 {
            1.0
        } else {
            -1.0
        }
    }

    /// Returns the number of support vectors
    ///
    /// Support vectors are observations with non-zero lambda values
    pub fn n_support_vectors(&self) -> usize {
        self.optimum.iter().filter(|&&l| l > 1e-5).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_linear_kernel_simple() {
        let observations = vec![
            array![0.0, 1.0],
            array![0.0, 2.0],
            array![1.0, 1.0],
            array![1.0, 2.0],
        ];
        let classes = array![1.0, 1.0, -1.0, -1.0];

        let mut svc = SVC::new(Kernel::Linear, f64::INFINITY).unwrap();
        svc.fit(&observations, &classes).unwrap();

        assert_eq!(svc.predict(&array![0.0, 1.0]), 1.0);
        assert_eq!(svc.predict(&array![1.0, 1.0]), -1.0);
        assert_eq!(svc.predict(&array![2.0, 2.0]), -1.0);
    }

    #[test]
    fn test_rbf_kernel() {
        let observations = vec![
            array![0.0, 0.0],
            array![1.0, 1.0],
            array![0.0, 1.0],
            array![1.0, 0.0],
        ];
        let classes = array![1.0, 1.0, -1.0, -1.0];

        let mut svc = SVC::new(Kernel::Rbf { gamma: 1.0 }, 1.0).unwrap();
        svc.fit(&observations, &classes).unwrap();

        // The RBF kernel should handle this XOR-like pattern better than linear
        assert_eq!(svc.predict(&array![0.0, 0.0]), 1.0);
        assert_eq!(svc.predict(&array![1.0, 1.0]), 1.0);
    }

    #[test]
    fn test_invalid_gamma() {
        let result = SVC::new(Kernel::Rbf { gamma: -1.0 }, 1.0);
        assert!(matches!(result, Err(SVCError::InvalidGamma)));

        let result = SVC::new(Kernel::Rbf { gamma: 0.0 }, 1.0);
        assert!(matches!(result, Err(SVCError::InvalidGamma)));
    }

    #[test]
    fn test_invalid_regularization() {
        let result = SVC::new(Kernel::Linear, 0.0);
        assert!(matches!(result, Err(SVCError::InvalidRegularization)));

        let result = SVC::new(Kernel::Linear, -1.0);
        assert!(matches!(result, Err(SVCError::InvalidRegularization)));
    }

    #[test]
    fn test_empty_data() {
        let mut svc = SVC::new(Kernel::Linear, 1.0).unwrap();
        let result = svc.fit(&[], &Array1::zeros(0));
        assert!(matches!(result, Err(SVCError::EmptyData)));
    }

    #[test]
    fn test_mismatched_dimensions() {
        let mut svc = SVC::new(Kernel::Linear, 1.0).unwrap();
        let observations = vec![array![1.0, 2.0]];
        let classes = array![1.0, -1.0]; // Too many classes
        let result = svc.fit(&observations, &classes);
        assert!(matches!(result, Err(SVCError::MismatchedDimensions)));
    }

    #[test]
    fn test_support_vectors_count() {
        let observations = vec![
            array![0.0, 1.0],
            array![0.0, 2.0],
            array![1.0, 1.0],
            array![1.0, 2.0],
        ];
        let classes = array![1.0, 1.0, -1.0, -1.0];

        let mut svc = SVC::new(Kernel::Linear, f64::INFINITY).unwrap();
        svc.fit(&observations, &classes).unwrap();

        // Should have at least some support vectors
        assert!(svc.n_support_vectors() > 0);
        assert!(svc.n_support_vectors() <= observations.len());
    }
}
