/* auto-exports start */
mod hinge_loss;
mod kl_divergence_loss;
mod mean_absolute_error_loss;
mod mean_squared_error_loss;

pub use hinge_loss::hng_loss;
pub use kl_divergence_loss::kld_loss;
pub use mean_absolute_error_loss::mae_loss;
pub use mean_squared_error_loss::mse_loss;
/* auto-exports end */
