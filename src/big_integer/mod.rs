#![cfg(feature = "big-math")]

mod fast_factorial;
mod multiply;
mod poly1305;

pub use self::fast_factorial::fast_factorial;
pub use self::multiply::multiply;
pub use self::poly1305::Poly1305;
