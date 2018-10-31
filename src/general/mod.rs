mod hanoi;
mod kmeans;

pub use self::hanoi::hanoi;
pub use self::kmeans::f32::kmeans as kmeans_f32;
pub use self::kmeans::f64::kmeans as kmeans_f64;
