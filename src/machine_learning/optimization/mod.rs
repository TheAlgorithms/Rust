mod adam;
mod gradient_descent;
mod k_nearest_neighbors;

pub use self::adam::Adam;
pub use self::gradient_descent::gradient_descent;
pub use self::k_nearest_neighbors::classify_using_knn;
