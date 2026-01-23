mod cholesky;
mod decision_tree;
mod k_means;
mod k_nearest_neighbors;
mod linear_regression;
mod logistic_regression;
mod loss_function;
mod naive_bayes;
mod optimization;
mod perceptron;
mod principal_component_analysis;
mod random_forest;
mod support_vector_classifier;

pub use self::cholesky::cholesky;
pub use self::decision_tree::decision_tree;
pub use self::k_means::k_means;
pub use self::k_nearest_neighbors::k_nearest_neighbors;
pub use self::linear_regression::linear_regression;
pub use self::logistic_regression::logistic_regression;
pub use self::loss_function::{
    average_margin_ranking_loss, hng_loss, huber_loss, kld_loss, mae_loss, mse_loss,
    neg_log_likelihood,
};
pub use self::naive_bayes::naive_bayes;
pub use self::optimization::{gradient_descent, Adam};
pub use self::perceptron::{classify, perceptron};
pub use self::principal_component_analysis::principal_component_analysis;
pub use self::random_forest::random_forest;
pub use self::support_vector_classifier::{Kernel, SVCError, SVC};
