//! LZ77 Compression Algorithm
//!
//! LZ77 is a lossless data compression algorithm published by Abraham Lempel and Jacob Ziv in 1977.
//! Also known as LZ1 or sliding-window compression, it forms the basis for many variations
//! including LZW, LZSS, LZMA and others.
//!
//! # Algorithm Overview
//!
//! It uses a "sliding window" method where the window contains:
//! - Search buffer: previously seen data that can be referenced
//! - Look-ahead buffer: data currently being encoded
//!
//! LZ77 encodes data using triplets (tokens) composed of:
//! - **Offset**: distance from the current position to the start of a match in the search buffer
//! - **Length**: number of characters that match
//! - **Indicator**: the next character to be encoded
//!
//! # Examples
//!
//! ```
//! use the_algorithms_rust::compression::LZ77Compressor;
//!
//! let compressor = LZ77Compressor::new(13, 6);
//! let text = "ababcbababaa";
//! let compressed = compressor.compress(text);
//! let decompressed = compressor.decompress(&compressed);
//! assert_eq!(text, decompressed);
//! ```
//!
//! # References
//!
//! - [Wikipedia: LZ77 and LZ78](https://en.wikipedia.org/wiki/LZ77_and_LZ78)

use std::fmt;

/// Represents a compression token (triplet) used in LZ77 compression.
///
/// A token consists of:
/// - `offset`: distance to the start of the match in the search buffer
/// - `length`: number of matching characters
/// - `indicator`: the next character to be encoded
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub offset: usize,
    pub length: usize,
    pub indicator: char,
}

impl Token {
    /// Creates a new Token.
    pub fn new(offset: usize, length: usize, indicator: char) -> Self {
        Self {
            offset,
            length,
            indicator,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.offset, self.length, self.indicator)
    }
}

/// LZ77 Compressor with configurable window and lookahead buffer sizes.
#[derive(Debug, Clone)]
pub struct LZ77Compressor {
    search_buffer_size: usize,
}

impl LZ77Compressor {
    /// Creates a new LZ77Compressor with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `window_size` - Total size of the sliding window
    /// * `lookahead_buffer_size` - Size of the lookahead buffer
    ///
    /// # Panics
    ///
    /// Panics if `lookahead_buffer_size` is greater than or equal to `window_size`.
    ///
    /// # Examples
    ///
    /// ```
    /// use the_algorithms_rust::compression::LZ77Compressor;
    ///
    /// let compressor = LZ77Compressor::new(13, 6);
    /// ```
    pub fn new(window_size: usize, lookahead_buffer_size: usize) -> Self {
        assert!(
            lookahead_buffer_size < window_size,
            "lookahead_buffer_size must be less than window_size"
        );

        Self {
            search_buffer_size: window_size - lookahead_buffer_size,
        }
    }

    /// Compresses the given text using the LZ77 algorithm.
    ///
    /// # Arguments
    ///
    /// * `text` - The string to be compressed
    ///
    /// # Returns
    ///
    /// A vector of `Token`s representing the compressed data
    ///
    /// # Examples
    ///
    /// ```
    /// use the_algorithms_rust::compression::LZ77Compressor;
    ///
    /// let compressor = LZ77Compressor::new(13, 6);
    /// let compressed = compressor.compress("ababcbababaa");
    /// assert_eq!(compressed.len(), 5);
    /// ```
    pub fn compress(&self, text: &str) -> Vec<Token> {
        let mut output = Vec::new();
        let mut search_buffer = String::new();
        let mut remaining_text = text.to_string();

        while !remaining_text.is_empty() {
            // Find the next encoding token
            let token = self.find_encoding_token(&remaining_text, &search_buffer);

            // Update the search buffer with the newly processed characters
            let chars_to_add = token.length + 1;
            let new_chars: String = remaining_text.chars().take(chars_to_add).collect();
            search_buffer.push_str(&new_chars);

            // Trim search buffer if it exceeds the maximum size
            if search_buffer.len() > self.search_buffer_size {
                let trim_amount = search_buffer.len() - self.search_buffer_size;
                search_buffer = search_buffer.chars().skip(trim_amount).collect();
            }

            // Remove processed characters from remaining text
            remaining_text = remaining_text.chars().skip(chars_to_add).collect();

            // Add token to output
            output.push(token);
        }

        output
    }

    /// Decompresses a list of tokens back into the original text.
    ///
    /// # Arguments
    ///
    /// * `tokens` - A slice of `Token`s representing compressed data
    ///
    /// # Returns
    ///
    /// The decompressed string
    ///
    /// # Examples
    ///
    /// ```
    /// use the_algorithms_rust::compression::{LZ77Compressor, Token};
    ///
    /// let compressor = LZ77Compressor::new(13, 6);
    /// let tokens = vec![
    ///     Token::new(0, 0, 'a'),
    ///     Token::new(0, 0, 'b'),
    ///     Token::new(2, 2, 'c'),
    ///     Token::new(4, 3, 'a'),
    ///     Token::new(2, 2, 'a'),
    /// ];
    /// let decompressed = compressor.decompress(&tokens);
    /// assert_eq!(decompressed, "ababcbababaa");
    /// ```
    pub fn decompress(&self, tokens: &[Token]) -> String {
        let mut output = String::new();

        for token in tokens {
            // Copy characters from the existing output based on offset and length
            for _ in 0..token.length {
                let index = output.len() - token.offset;
                let ch = output.chars().nth(index).unwrap();
                output.push(ch);
            }
            // Add the indicator character
            output.push(token.indicator);
        }

        output
    }

    /// Finds the encoding token for the current position in the text.
    ///
    /// This method searches the search buffer for the longest match with the
    /// beginning of the text and returns the corresponding token.
    fn find_encoding_token(&self, text: &str, search_buffer: &str) -> Token {
        if text.is_empty() {
            panic!("Cannot encode empty text");
        }

        let mut length = 0;
        let mut offset = 0;

        if search_buffer.is_empty() {
            return Token::new(offset, length, text.chars().next().unwrap());
        }

        let search_chars: Vec<char> = search_buffer.chars().collect();
        let text_chars: Vec<char> = text.chars().collect();

        // We must keep at least one character for the indicator
        let max_match_length = text_chars.len() - 1;

        // Search for matches in the search buffer
        for (i, &ch) in search_chars.iter().enumerate() {
            let found_offset = search_chars.len() - i;

            if ch == text_chars[0] {
                let found_length = Self::match_length_from_index(&text_chars, &search_chars, 0, i);

                // Limit match length to ensure we have an indicator character
                let found_length = found_length.min(max_match_length);

                // Update if we found a longer match (or same length with smaller offset)
                if found_length >= length {
                    offset = found_offset;
                    length = found_length;
                }
            }
        }

        Token::new(offset, length, text_chars[length])
    }

    /// Calculates the length of the longest match between text and window
    /// starting from the given indices.
    ///
    /// This is a helper function that recursively finds matching characters.
    fn match_length_from_index(
        text: &[char],
        window: &[char],
        text_index: usize,
        window_index: usize,
    ) -> usize {
        // Base cases
        if text_index >= text.len() || window_index >= window.len() {
            return 0;
        }

        if text[text_index] != window[window_index] {
            return 0;
        }

        // Recursive case: characters match, continue checking
        let mut extended_window = window.to_vec();
        extended_window.push(text[text_index]);

        1 + Self::match_length_from_index(text, &extended_window, text_index + 1, window_index + 1)
    }
}

impl Default for LZ77Compressor {
    /// Creates a default LZ77Compressor with window_size=13 and lookahead_buffer_size=6.
    fn default() -> Self {
        Self::new(13, 6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_display() {
        let token = Token::new(1, 2, 'c');
        assert_eq!(token.to_string(), "(1, 2, c)");
    }

    #[test]
    fn test_compress_ababcbababaa() {
        let compressor = LZ77Compressor::new(13, 6);
        let compressed = compressor.compress("ababcbababaa");

        let expected = vec![
            Token::new(0, 0, 'a'),
            Token::new(0, 0, 'b'),
            Token::new(2, 2, 'c'),
            Token::new(4, 3, 'a'),
            Token::new(2, 2, 'a'),
        ];

        assert_eq!(compressed, expected);
    }

    #[test]
    fn test_compress_aacaacabcabaaac() {
        let compressor = LZ77Compressor::new(13, 6);
        let compressed = compressor.compress("aacaacabcabaaac");

        let expected = vec![
            Token::new(0, 0, 'a'),
            Token::new(1, 1, 'c'),
            Token::new(3, 4, 'b'),
            Token::new(3, 3, 'a'),
            Token::new(1, 2, 'c'),
        ];

        assert_eq!(compressed, expected);
    }

    #[test]
    fn test_decompress_cabracadabrarrarrad() {
        let compressor = LZ77Compressor::new(13, 6);
        let tokens = vec![
            Token::new(0, 0, 'c'),
            Token::new(0, 0, 'a'),
            Token::new(0, 0, 'b'),
            Token::new(0, 0, 'r'),
            Token::new(3, 1, 'c'),
            Token::new(2, 1, 'd'),
            Token::new(7, 4, 'r'),
            Token::new(3, 5, 'd'),
        ];

        let decompressed = compressor.decompress(&tokens);
        assert_eq!(decompressed, "cabracadabrarrarrad");
    }

    #[test]
    fn test_decompress_ababcbababaa() {
        let compressor = LZ77Compressor::new(13, 6);
        let tokens = vec![
            Token::new(0, 0, 'a'),
            Token::new(0, 0, 'b'),
            Token::new(2, 2, 'c'),
            Token::new(4, 3, 'a'),
            Token::new(2, 2, 'a'),
        ];

        let decompressed = compressor.decompress(&tokens);
        assert_eq!(decompressed, "ababcbababaa");
    }

    #[test]
    fn test_decompress_aacaacabcabaaac() {
        let compressor = LZ77Compressor::new(13, 6);
        let tokens = vec![
            Token::new(0, 0, 'a'),
            Token::new(1, 1, 'c'),
            Token::new(3, 4, 'b'),
            Token::new(3, 3, 'a'),
            Token::new(1, 2, 'c'),
        ];

        let decompressed = compressor.decompress(&tokens);
        assert_eq!(decompressed, "aacaacabcabaaac");
    }

    #[test]
    fn test_round_trip_compression() {
        let compressor = LZ77Compressor::new(13, 6);
        let texts = vec![
            "cabracadabrarrarrad",
            "ababcbababaa",
            "aacaacabcabaaac",
            "hello world",
            "aaaaaaa",
            "abcdefghijk",
        ];

        for text in texts {
            let compressed = compressor.compress(text);
            let decompressed = compressor.decompress(&compressed);
            assert_eq!(text, decompressed, "Round trip failed for text: {}", text);
        }
    }

    #[test]
    fn test_empty_search_buffer() {
        let compressor = LZ77Compressor::new(13, 6);
        let token = compressor.find_encoding_token("abc", "");
        assert_eq!(token, Token::new(0, 0, 'a'));
    }

    #[test]
    #[should_panic(expected = "Cannot encode empty text")]
    fn test_empty_text_panics() {
        let compressor = LZ77Compressor::new(13, 6);
        compressor.find_encoding_token("", "xyz");
    }

    #[test]
    fn test_default_compressor() {
        let compressor = LZ77Compressor::default();
        let text = "test";
        let compressed = compressor.compress(text);
        let decompressed = compressor.decompress(&compressed);
        assert_eq!(text, decompressed);
    }

    #[test]
    #[should_panic(expected = "lookahead_buffer_size must be less than window_size")]
    fn test_invalid_buffer_sizes() {
        LZ77Compressor::new(10, 10);
    }

    #[test]
    fn test_single_character() {
        let compressor = LZ77Compressor::new(13, 6);
        let compressed = compressor.compress("a");
        assert_eq!(compressed, vec![Token::new(0, 0, 'a')]);
        let decompressed = compressor.decompress(&compressed);
        assert_eq!(decompressed, "a");
    }

    #[test]
    fn test_repeated_pattern() {
        let compressor = LZ77Compressor::new(13, 6);
        let text = "abababab";
        let compressed = compressor.compress(text);
        let decompressed = compressor.decompress(&compressed);
        assert_eq!(text, decompressed);
    }
}
