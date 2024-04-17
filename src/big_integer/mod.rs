#![cfg(feature = "big-math")]

/* auto-exports start */
mod fast_factorial;
mod poly1305;

pub use fast_factorial::fast_factorial;
pub use poly1305::Poly1305;
/* auto-exports end */
