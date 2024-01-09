// Given a string and a list of words, return true if the string can be
// segmented into a space-separated sequence of one or more words.

// Note that the same word may be reused
// multiple times in the segmentation.

// Implementation notes: Trie + Dynamic programming up -> down.
// The Trie will be used to store the words. It will be useful for scanning
// available words for the current position in the string.

use std::collections::HashMap;

struct Trie {
    children: HashMap<char, Box<Trie>>,
    is_word: bool,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            children: HashMap::new(),
            is_word: false,
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node
                .children
                .entry(c)
                .or_insert_with(|| Box::new(Trie::new()));
        }
        node.is_word = true;
    }

    pub fn search(&self, s: &str, start: usize, memo: &mut Vec<Option<bool>>) -> bool {
        if start >= s.len() {
            return true;
        }

        if let Some(res) = memo[start] {
            return res;
        }

        let mut node = self;
        for (i, c) in s[start..].chars().enumerate() {
            if let Some(n) = node.children.get(&c) {
                node = n;
                if node.is_word && self.search(s, start + i + 1, memo) {
                    memo[start] = Some(true);
                    return true;
                }
            } else {
                break;
            }
        }

        memo[start] = Some(false);
        false
    }
}

pub fn word_break(s: &str, word_dict: Vec<&str>) -> bool {
    let mut trie = Trie::new();
    for word in word_dict {
        trie.insert(word);
    }

    let mut memo = vec![None; s.len()];
    trie.search(s, 0, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::word_break;

    #[test]
    fn typical_cases() {
        assert!(word_break("applepenapple", vec!["apple", "pen"]));
        assert!(!word_break(
            "catsandog",
            vec!["cats", "dog", "sand", "and", "cat"]
        ));
        assert!(word_break("cars", vec!["car", "ca", "rs"]));
    }

    #[test]
    fn edge_cases() {
        assert!(!word_break("abc", vec![]));
        assert!(word_break("a", vec!["a"]));
    }

    #[test]
    fn repeated_words() {
        assert!(word_break("aabb", vec!["a", "b"]));
        assert!(word_break("aaaaaaa", vec!["a", "aa", "aaa"]));
    }

    #[test]
    fn no_solution() {
        assert!(!word_break("abcdef", vec!["ab", "abc", "cd"]));
        assert!(!word_break("xyz", vec!["a", "b", "c"]));
    }

    #[test]
    fn long_string() {
        let long_string = "a".repeat(100);
        let words = vec!["a", "aa", "aaa", "aaaa"];
        assert!(word_break(&long_string, words));
    }
}
