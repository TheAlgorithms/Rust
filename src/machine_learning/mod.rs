mod linear_regression;
mod loss_function;
mod optimization;

pub use self::linear_regression::linear_regression;
pub use self::loss_function::mse_loss;
pub use self::optimization::gradient_descent;
