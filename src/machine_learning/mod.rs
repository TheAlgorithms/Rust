mod cholesky;
mod k_means;
mod linear_regression;
mod logistic_regression;
mod loss_function;
mod optimization;
mod k_nearest_neighbors;


pub use self::cholesky::cholesky;
pub use self::k_means::k_means;
pub use self::linear_regression::linear_regression;
pub use self::logistic_regression::logistic_regression;
pub use self::loss_function::average_margin_ranking_loss;
pub use self::loss_function::hng_loss;
pub use self::loss_function::huber_loss;
pub use self::loss_function::kld_loss;
pub use self::loss_function::mae_loss;
pub use self::loss_function::mse_loss;
pub use self::loss_function::neg_log_likelihood;
pub use self::optimization::gradient_descent;
pub use self::optimization::Adam;
pub use self::k_nearest_neighbors::{DataPoint, KNearestNeighbors};

