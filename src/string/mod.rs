mod aho_corasick;
mod anagram;
mod autocomplete_using_trie;
mod boyer_moore_search;
mod burrows_wheeler_transform;
mod duval_algorithm;
mod hamming_distance;
mod jaro_winkler_distance;
mod knuth_morris_pratt;
mod levenshtein_distance;
mod lipogram;
mod manacher;
mod palindrome;
mod pangram;
mod rabin_karp;
mod reverse;
mod run_length_encoding;
mod suffix_array;
mod suffix_array_manber_myers;
mod suffix_tree;
mod z_algorithm;

pub use self::aho_corasick::AhoCorasick;
pub use self::anagram::check_anagram;
pub use self::autocomplete_using_trie::Autocomplete;
pub use self::boyer_moore_search::boyer_moore_search;
pub use self::burrows_wheeler_transform::{
    burrows_wheeler_transform, inv_burrows_wheeler_transform,
};
pub use self::duval_algorithm::duval_algorithm;
pub use self::hamming_distance::hamming_distance;
pub use self::jaro_winkler_distance::jaro_winkler_distance;
pub use self::knuth_morris_pratt::knuth_morris_pratt;
pub use self::levenshtein_distance::{
    naive_levenshtein_distance, optimized_levenshtein_distance
};
pub use self::lipogram::is_lipogram;
pub use self::manacher::manacher;
pub use self::palindrome::is_palindrome;
pub use self::pangram::is_pangram;
pub use self::pangram::PangramStatus;
pub use self::rabin_karp::rabin_karp;
pub use self::reverse::reverse;
pub use self::run_length_encoding::{run_length_decoding, run_length_encoding};
pub use self::suffix_array::generate_suffix_array;
pub use self::suffix_array_manber_myers::generate_suffix_array_manber_myers;
pub use self::suffix_tree::{Node, SuffixTree};
pub use self::z_algorithm::match_pattern;
pub use self::z_algorithm::z_array;
