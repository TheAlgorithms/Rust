/*
    It autocomplete by prefix using added words.

    word List => ["apple", "orange", "oregano"]
    prefix => "or"
    matches => ["orange", "oregano"]
*/

use std::collections::HashMap;

const END: char = '#';

#[derive(Debug)]
struct Trie(HashMap<char, Box<Trie>>);

impl Trie {
    fn new() -> Self {
        Trie(HashMap::new())
    }

    fn insert(&mut self, text: String) {
        let mut trie = self;

        for c in text.chars().collect::<Vec<char>>() {
            trie = trie.0.entry(c).or_insert_with(|| Box::new(Trie::new()));
        }

        trie.0.insert(END, Box::new(Trie::new()));
    }

    fn find(&self, prefix: String) -> Vec<String> {
        let mut trie = self;

        for c in prefix.chars().collect::<Vec<char>>() {
            let char_trie = trie.0.get(&c);
            if let Some(char_trie) = char_trie {
                trie = char_trie;
            } else {
                return vec![];
            }
        }

        self._elements(trie)
            .iter()
            .map(|s| prefix.clone() + s)
            .collect()
    }

    fn _elements(&self, map: &Trie) -> Vec<String> {
        let mut results = vec![];

        for (c, v) in map.0.iter() {
            let mut sub_result = vec![];
            if c == &END {
                sub_result.push("".to_owned())
            } else {
                for s in self._elements(v) {
                    let res_string = c.to_string() + &s;
                    sub_result.push(res_string);
                }
            }

            results.extend(sub_result)
        }

        results
    }
}

pub struct Autocomplete {
    trie: Trie,
}

impl Autocomplete {
    fn new() -> Self {
        Self { trie: Trie::new() }
    }

    pub fn insert_words(&mut self, words: Vec<String>) {
        for word in words {
            self.trie.insert(word);
        }
    }

    pub fn find_words(&self, prefix: String) -> Vec<String> {
        self.trie.find(prefix)
    }
}

impl Default for Autocomplete {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Autocomplete;

    #[test]
    fn test_autocomplete() {
        let words = vec![
            "apple".to_owned(),
            "orange".to_owned(),
            "oregano".to_owned(),
        ];

        let mut auto_complete = Autocomplete::new();
        auto_complete.insert_words(words);

        let prefix = "app".to_owned();
        let mut auto_completed_words = auto_complete.find_words(prefix);

        assert_eq!(auto_completed_words.sort(), vec!["apple".to_owned()].sort());

        let prefix = "or".to_owned();
        let mut auto_completed_words = auto_complete.find_words(prefix);

        assert_eq!(
            auto_completed_words.sort(),
            vec!["orange".to_owned(), "oregano".to_owned()].sort()
        );
    }
}
