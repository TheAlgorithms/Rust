/**
 * Text Search in Rust
 *
 * This code provides a simple text search functionality in Rust. It allows you to search for a word
 * in a given paragraph, with an option to perform case-insensitive search. The code includes a set
 * of test cases to ensure the correctness of the search functionality.
 *
 * Problem Solved:
 * - This code addresses the problem of searching for words within a text paragraph.
 * - It solves the problem of performing both case-sensitive and case-insensitive searches.
 *
 * Usage:
 * - You can use the `search_word` function to search for a word in a paragraph.
 * - You can use the `ignore_case` parameter to control case sensitivity in the search.
 * - A set of test cases is provided to ensure the correctness of the search functionality.
 */

 pub fn search_word(paragraph: &str, word: &str, ignore_case: bool) -> Option<usize> {
    // Convert the paragraph and word to lowercase if case-insensitive search is required
    let paragraph = if ignore_case {
        paragraph.to_lowercase()
    } else {
        paragraph.to_string()
    };

    let word = if ignore_case {
        word.to_lowercase()
    } else {
        word.to_string()
    };

    // Use the find method to search for the word in the paragraph
    paragraph.find(&word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_case_insensitive() {
        let paragraph = "This is a case-insensitive test. Case-Insensitive search is working.";
        let word = "Case-Insensitive";
        let ignore_case = true;
        assert_eq!(search_word(paragraph, word, ignore_case), Some(10));
    }

    #[test]
    fn test_search_case_sensitive() {
        let paragraph = "This is a case-sensitive test. Case-Sensitive search is working.";
        let word = "Case-Sensitive";
        let ignore_case = true;
        assert_eq!(search_word(paragraph, word, ignore_case), Some(10));
    }

    #[test]
    fn test_search_not_found() {
        let paragraph = "This is a test paragraph.";
        let word = "Not Found";
        let ignore_case = true;
        assert_eq!(search_word(paragraph, word, ignore_case), None);
    }

    #[test]
    fn test_search_edge_case() {
        let paragraph = "Test this test paragraph with 'test' word multiple times. Test.";
        let word = "test";
        let ignore_case = true;
        assert_eq!(search_word(paragraph, word, ignore_case), Some(0));
    }

    #[test]
    fn test_search_empty_paragraph() {
        let paragraph = "";
        let word = "Empty";
        let ignore_case = true;
        assert_eq!(search_word(paragraph, word, ignore_case), None);
    }

    #[test]
    fn test_search_multiple_occurrences() {
        let paragraph = "This is a test paragraph. This paragraph contains multiple occurrences of the word 'test'.";
        let word = "test";
        let ignore_case = true;
        assert_eq!(search_word(paragraph, word, ignore_case), Some(10));
    }

    #[test]
    fn test_search_word_with_punctuation() {
        let paragraph = "This is a test, including some punctuation. Testing: 1, 2, 3.";
        let word = "testing";
        let ignore_case = true;
        assert_eq!(search_word(paragraph, word, ignore_case), Some(44));
    }
}
