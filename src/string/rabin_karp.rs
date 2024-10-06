//! This module implements the Rabin-Karp string searching algorithm.
//! It uses a rolling hash technique to find all occurrences of a pattern
//! within a target string efficiently.

const MOD: usize = 101;
const RADIX: usize = 256;

/// Finds all starting indices where the `pattern` appears in the `text`.
///
/// # Arguments
/// * `text` - The string where the search is performed.
/// * `pattern` - The substring pattern to search for.
///
/// # Returns
/// A vector of starting indices where the pattern is found.
pub fn rabin_karp(text: &str, pattern: &str) -> Vec<usize> {
    if text.is_empty() || pattern.is_empty() || pattern.len() > text.len() {
        return vec![];
    }

    let pat_hash = compute_hash(pattern);
    let mut radix_pow = 1;

    // Compute RADIX^(n-1) % MOD
    for _ in 0..pattern.len() - 1 {
        radix_pow = (radix_pow * RADIX) % MOD;
    }

    let mut rolling_hash = 0;
    let mut result = vec![];
    for i in 0..=text.len() - pattern.len() {
        rolling_hash = if i == 0 {
            compute_hash(&text[0..pattern.len()])
        } else {
            update_hash(text, i - 1, i + pattern.len() - 1, rolling_hash, radix_pow)
        };
        if rolling_hash == pat_hash && pattern[..] == text[i..i + pattern.len()] {
            result.push(i);
        }
    }
    result
}

/// Calculates the hash of a string using the Rabin-Karp formula.
///
/// # Arguments
/// * `s` - The string to calculate the hash for.
///
/// # Returns
/// The hash value of the string modulo `MOD`.
fn compute_hash(s: &str) -> usize {
    let mut hash_val = 0;
    for &byte in s.as_bytes().iter() {
        hash_val = (hash_val * RADIX + byte as usize) % MOD;
    }
    hash_val
}

/// Updates the rolling hash when shifting the search window.
///
/// # Arguments
/// * `s` - The full text where the search is performed.
/// * `old_idx` - The index of the character that is leaving the window.
/// * `new_idx` - The index of the new character entering the window.
/// * `old_hash` - The hash of the previous substring.
/// * `radix_pow` - The precomputed value of RADIX^(n-1) % MOD.
///
/// # Returns
/// The updated hash for the new substring.
fn update_hash(
    s: &str,
    old_idx: usize,
    new_idx: usize,
    old_hash: usize,
    radix_pow: usize,
) -> usize {
    let mut new_hash = old_hash;
    let old_char = s.as_bytes()[old_idx] as usize;
    let new_char = s.as_bytes()[new_idx] as usize;
    new_hash = (new_hash + MOD - (old_char * radix_pow % MOD)) % MOD;
    new_hash = (new_hash * RADIX + new_char) % MOD;
    new_hash
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_cases {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (text, pattern, expected) = $inputs;
                    assert_eq!(rabin_karp(text, pattern), expected);
                }
            )*
        };
    }

    test_cases! {
        single_match_at_start: ("hello world", "hello", vec![0]),
        single_match_at_end: ("hello world", "world", vec![6]),
        single_match_in_middle: ("abc def ghi", "def", vec![4]),
        multiple_matches: ("ababcabc", "abc", vec![2, 5]),
        overlapping_matches: ("aaaaa", "aaa", vec![0, 1, 2]),
        no_match: ("abcdefg", "xyz", vec![]),
        pattern_is_entire_string: ("abc", "abc", vec![0]),
        target_is_multiple_patterns: ("abcabcabc", "abc", vec![0, 3, 6]),
        empty_text: ("", "abc", vec![]),
        empty_pattern: ("abc", "", vec![]),
        pattern_larger_than_text: ("abc", "abcd", vec![]),
        single_char_match: ("a", "a", vec![0]),
        single_char_no_match: ("a", "b", vec![]),
        large_pattern_no_match: ("abc", "defghi", vec![]),
        repeating_chars: ("aaaaaa", "aa", vec![0, 1, 2, 3, 4]),
        special_characters: ("abc$def@ghi", "$def@", vec![3]),
        numeric_and_alphabetic_mix: ("abc123abc456", "123abc", vec![3]),
        case_sensitivity: ("AbcAbc", "abc", vec![]),
    }
}
