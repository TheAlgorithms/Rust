/* auto-exports start */
pub mod loss_function;
pub mod optimization;

mod cholesky;
mod k_means;
mod linear_regression;

pub use cholesky::cholesky;
pub use k_means::k_means;
pub use linear_regression::linear_regression;
/* auto-exports end */
