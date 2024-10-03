use crate::data_structures::Trie;

/// Checks if a string can be segmented into a space-separated sequence
/// of one or more words from the given dictionary.
///
/// # Arguments
/// * `s` - The input string to be segmented.
/// * `word_dict` - A list of words forming the dictionary.
///
/// # Returns
/// * `bool` - `true` if the string can be segmented, `false` otherwise.
pub fn word_break(s: &str, word_dict: Vec<&str>) -> bool {
    let mut trie = Trie::new();
    for word in word_dict {
        trie.insert(word.chars(), true);
    }

    let mut memo = vec![None; s.len()];
    search(&trie, s, 0, &mut memo)
}

/// Recursively checks if the substring starting from `start` can be segmented
/// using words in the trie and memoizes the results.
///
/// # Arguments
/// * `trie` - The Trie containing the dictionary words.
/// * `s` - The input string.
/// * `start` - The starting index for the current substring.
/// * `memo` - A vector for memoization to store intermediate results.
///
/// # Returns
/// * `bool` - `true` if the substring can be segmented, `false` otherwise.
fn search(trie: &Trie<char, bool>, s: &str, start: usize, memo: &mut Vec<Option<bool>>) -> bool {
    if start >= s.len() {
        return true;
    }

    if let Some(res) = memo[start] {
        return res;
    }

    for end in start + 1..=s.len() {
        if trie.get(s[start..end].chars()).is_some() && search(trie, s, end, memo) {
            memo[start] = Some(true);
            return true;
        }
    }

    memo[start] = Some(false);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_cases {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, dict, expected) = $test_case;
                    assert_eq!(word_break(input, dict), expected);
                }
            )*
        }
    }

    test_cases! {
        typical_case_1: ("applepenapple", vec!["apple", "pen"], true),
        typical_case_2: ("catsandog", vec!["cats", "dog", "sand", "and", "cat"], false),
        typical_case_3: ("cars", vec!["car", "ca", "rs"], true),
        edge_case_1: ("abc", vec![], false),
        edge_case_2: ("a", vec!["a"], true),
        repeated_words_case_1: ("aabb", vec!["a", "b"], true),
        repeated_words_case_2: ("aaaaaaa", vec!["a", "aa", "aaa"], true),
        no_solution_case_1: ("abcdef", vec!["ab", "abc", "cd"], false),
        no_solution_case_2: ("xyz", vec!["a", "b", "c"], false),
        long_string_case: (&"a".repeat(100), vec!["a", "aa", "aaa", "aaaa"], true),
    }
}
