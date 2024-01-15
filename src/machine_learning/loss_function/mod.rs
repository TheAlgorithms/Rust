mod hinge_loss;
mod kl_divergence_loss;
mod mean_absolute_error_loss;
mod mean_squared_error_loss;

pub use self::hinge_loss::hng_loss;
pub use self::kl_divergence_loss::kld_loss;
pub use self::mean_absolute_error_loss::mae_loss;
pub use self::mean_squared_error_loss::mse_loss;
