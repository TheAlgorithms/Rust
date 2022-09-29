mod aho_corasick;
mod burrows_wheeler_transform;
mod hamming_distance;
mod knuth_morris_pratt;
mod manacher;
mod palindrome;
mod rabin_karp;
mod reverse;
mod run_length_encoding;
mod z_algorithm;

pub use self::aho_corasick::AhoCorasick;
pub use self::burrows_wheeler_transform::{
    burrows_wheeler_transform, inv_burrows_wheeler_transform,
};
pub use self::hamming_distance::hamming_distance;
pub use self::knuth_morris_pratt::knuth_morris_pratt;
pub use self::manacher::manacher;
pub use self::palindrome::is_palindrome;
pub use self::rabin_karp::rabin_karp;
pub use self::reverse::reverse;
pub use self::run_length_encoding::{run_length_decoding, run_length_encoding};
pub use self::z_algorithm::match_pattern;
pub use self::z_algorithm::z_array;
