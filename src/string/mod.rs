/* auto-exports start */
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

pub use aho_corasick::AhoCorasick;
pub use anagram::check_anagram;
pub use autocomplete_using_trie::Autocomplete;
pub use boyer_moore_search::boyer_moore_search;
pub use burrows_wheeler_transform::{
	burrows_wheeler_transform,
	inv_burrows_wheeler_transform
};
pub use duval_algorithm::duval_algorithm;
pub use hamming_distance::hamming_distance;
pub use jaro_winkler_distance::jaro_winkler_distance;
pub use knuth_morris_pratt::knuth_morris_pratt;
pub use levenshtein_distance::levenshtein_distance;
pub use lipogram::is_lipogram;
pub use manacher::manacher;
pub use palindrome::is_palindrome;
pub use pangram::{
	PangramStatus,
	is_pangram
};
pub use rabin_karp::rabin_karp;
pub use reverse::reverse;
pub use run_length_encoding::{
	run_length_encoding,
	run_length_decoding
};
pub use suffix_array::generate_suffix_array;
pub use suffix_array_manber_myers::generate_suffix_array_manber_myers;
pub use suffix_tree::{
	Node,
	SuffixTree
};
pub use z_algorithm::{
	z_array,
	match_pattern
};
/* auto-exports end */
