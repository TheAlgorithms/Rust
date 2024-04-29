mod run_length_encoding;
mod lz77;

pub use self::lz77::{lz77_encode, lz77_decode};
pub use self::run_length_encoding::{run_length_decode, run_length_encode};
