#![cfg(feature = "big-math")]

mod fast_factorial;
mod hello_bigmath;
mod poly1305;

pub use self::fast_factorial::fast_factorial;
pub use self::hello_bigmath::factorial;
pub use self::poly1305::Poly1305;
