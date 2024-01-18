mod huffman;
mod run_length_encoding;

pub use self::huffman::{generate_huffman_code, huffman_decode, huffman_encode};
pub use self::run_length_encoding::{run_length_decode, run_length_encode};
