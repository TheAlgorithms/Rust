//! Implementation of Duval's Algorithm to compute the standard factorization of a string
//! into Lyndon words. A Lyndon word is defined as a string that is strictly smaller
//! (lexicographically) than any of its nontrivial suffixes. This implementation operates
//! in linear time and space.

/// Performs Duval's algorithm to factorize a given string into its Lyndon words.
///
/// # Arguments
///
/// * `s` - A slice of characters representing the input string.
///
/// # Returns
///
/// A vector of strings, where each string is a Lyndon word, representing the factorization
/// of the input string.
///
/// # Time Complexity
///
/// The algorithm runs in O(n) time, where `n` is the length of the input string.
pub fn duval_algorithm(s: &str) -> Vec<String> {
    factorize_duval(&s.chars().collect::<Vec<char>>())
}

/// Helper function that takes a string slice, converts it to a vector of characters,
/// and then applies the Duval factorization algorithm to find the Lyndon words.
///
/// # Arguments
///
/// * `s` - A string slice representing the input text.
///
/// # Returns
///
/// A vector of strings, each representing a Lyndon word in the factorization.
fn factorize_duval(s: &[char]) -> Vec<String> {
    let mut start = 0;
    let mut factors: Vec<String> = Vec::new();

    while start < s.len() {
        let mut end = start + 1;
        let mut repeat = start;

        while end < s.len() && s[repeat] <= s[end] {
            if s[repeat] < s[end] {
                repeat = start;
            } else {
                repeat += 1;
            }
            end += 1;
        }

        while start <= repeat {
            factors.push(s[start..start + end - repeat].iter().collect::<String>());
            start += end - repeat;
        }
    }

    factors
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_duval_algorithm {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (text, expected) = $inputs;
                    assert_eq!(duval_algorithm(text), expected);
                }
            )*
        }
    }

    test_duval_algorithm! {
        multiple: ("abcdabcdababc", ["abcd", "abcd", "ababc"]),
        all: ("aaa", ["a", "a", "a"]),
        single: ("ababb", ["ababb"]),
        unicode: ("അഅഅ", ["അ", "അ", "അ"]),
        empty_string: ("", Vec::<String>::new()),
        single_char: ("x", ["x"]),
        palindrome: ("racecar", ["r", "acecar"]),
        long_repeating: ("aaaaaa", ["a", "a", "a", "a", "a", "a"]),
        mixed_repeating: ("ababcbabc", ["ababcbabc"]),
        non_repeating_sorted: ("abcdefg", ["abcdefg"]),
    }
}
