mod average_margin_ranking_loss;
mod hinge_loss;
mod huber_loss;
mod kl_divergence_loss;
mod mean_absolute_error_loss;
mod mean_squared_error_loss;
mod negative_log_likelihood;

pub use self::average_margin_ranking_loss::average_margin_ranking_loss;
pub use self::hinge_loss::hng_loss;
pub use self::huber_loss::huber_loss;
pub use self::kl_divergence_loss::kld_loss;
pub use self::mean_absolute_error_loss::mae_loss;
pub use self::mean_squared_error_loss::mse_loss;
pub use self::negative_log_likelihood::neg_log_likelihood;
