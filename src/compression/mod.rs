mod lz77;
mod run_length_encoding;

pub use self::lz77::{lz77_decode, lz77_encode};
pub use self::run_length_encoding::{run_length_decode, run_length_encode};
