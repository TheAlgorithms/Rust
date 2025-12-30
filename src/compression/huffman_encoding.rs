//! Huffman Encoding implementation
//!
//! Huffman coding is a lossless data compression algorithm that assigns variable-length codes
//! to characters based on their frequency of occurrence. Characters that occur more frequently
//! are assigned shorter codes, while less frequent characters get longer codes.
//!
//! # Algorithm Overview
//!
//! 1. Count the frequency of each character in the input
//! 2. Build a min-heap (priority queue) of nodes based on frequency
//! 3. Build the Huffman tree by repeatedly:
//!    - Remove two nodes with minimum frequency
//!    - Create a parent node with combined frequency
//!    - Insert the parent back into the heap
//! 4. Traverse the tree to assign binary codes to each character
//! 5. Encode the input using the generated codes
//!
//! # Time Complexity
//!
//! - Building frequency map: O(n) where n is input length
//! - Building Huffman tree: O(m log m) where m is number of unique characters
//! - Encoding: O(n)
//!
//! # Usage
//!
//! As a library:
//! ```no_run
//! use the_algorithms_rust::compression::huffman_encode;
//!
//! let text = "hello world";
//! let (encoded, codes) = huffman_encode(text);
//! println!("Original: {}", text);
//! println!("Encoded: {}", encoded);
//! ```
//!
//! As a command-line tool:
//! ```bash
//! rustc huffman_encoding.rs -o huffman
//! ./huffman input.txt
//! ```

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

#[cfg(not(test))]
use std::env;

/// Represents a node in the Huffman tree
#[derive(Debug, Eq, PartialEq)]
enum HuffmanNode {
    /// Leaf node containing a character and its frequency
    Leaf { character: char, frequency: usize },
    /// Internal node with combined frequency and left/right children
    Internal {
        frequency: usize,
        left: Box<HuffmanNode>,
        right: Box<HuffmanNode>,
    },
}

impl HuffmanNode {
    /// Returns the frequency of this node
    fn frequency(&self) -> usize {
        match self {
            HuffmanNode::Leaf { frequency, .. } | HuffmanNode::Internal { frequency, .. } => {
                *frequency
            }
        }
    }

    /// Creates a new leaf node
    fn new_leaf(character: char, frequency: usize) -> Self {
        HuffmanNode::Leaf {
            character,
            frequency,
        }
    }

    /// Creates a new internal node from two children
    fn new_internal(left: HuffmanNode, right: HuffmanNode) -> Self {
        let frequency = left.frequency() + right.frequency();
        HuffmanNode::Internal {
            frequency,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

/// Wrapper for HuffmanNode to implement Ord for BinaryHeap (min-heap)
#[derive(Eq, PartialEq)]
struct HeapNode(HuffmanNode);

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.0.frequency().cmp(&self.0.frequency())
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Counts the frequency of each character in the input string
///
/// # Arguments
///
/// * `text` - The input string to analyze
///
/// # Returns
///
/// A HashMap mapping each character to its frequency count
fn build_frequency_map(text: &str) -> HashMap<char, usize> {
    let mut frequencies = HashMap::new();
    for ch in text.chars() {
        *frequencies.entry(ch).or_insert(0) += 1;
    }
    frequencies
}

/// Builds the Huffman tree from a frequency map
///
/// # Arguments
///
/// * `frequencies` - HashMap of character frequencies
///
/// # Returns
///
/// The root node of the Huffman tree, or None if input is empty
fn build_huffman_tree(frequencies: HashMap<char, usize>) -> Option<HuffmanNode> {
    if frequencies.is_empty() {
        return None;
    }

    let mut heap: BinaryHeap<HeapNode> = frequencies
        .into_iter()
        .map(|(ch, freq)| HeapNode(HuffmanNode::new_leaf(ch, freq)))
        .collect();

    // Special case: only one unique character
    if heap.len() == 1 {
        return heap.pop().map(|node| node.0);
    }

    // Build the tree by combining nodes
    while heap.len() > 1 {
        let left = heap.pop().unwrap().0;
        let right = heap.pop().unwrap().0;
        let parent = HuffmanNode::new_internal(left, right);
        heap.push(HeapNode(parent));
    }

    heap.pop().map(|node| node.0)
}

/// Traverses the Huffman tree to generate binary codes for each character
///
/// # Arguments
///
/// * `node` - The current node being traversed
/// * `code` - The current binary code string
/// * `codes` - HashMap to store the generated codes
fn generate_codes(node: &HuffmanNode, code: String, codes: &mut HashMap<char, String>) {
    match node {
        HuffmanNode::Leaf { character, .. } => {
            // Use "0" for single character case
            codes.insert(
                *character,
                if code.is_empty() {
                    "0".to_string()
                } else {
                    code
                },
            );
        }
        HuffmanNode::Internal { left, right, .. } => {
            generate_codes(left, format!("{code}0"), codes);
            generate_codes(right, format!("{code}1"), codes);
        }
    }
}

/// Encodes text using Huffman coding
///
/// # Arguments
///
/// * `text` - The input string to encode
///
/// # Returns
///
/// A tuple containing:
/// - The encoded binary string
/// - A HashMap of character to binary code mappings
///
/// # Examples
///
/// ```
/// # use std::collections::HashMap;
/// # use the_algorithms_rust::compression::huffman_encode;
/// let (encoded, codes) = huffman_encode("hello");
/// assert!(!encoded.is_empty());
/// assert!(codes.contains_key(&'h'));
/// ```
pub fn huffman_encode(text: &str) -> (String, HashMap<char, String>) {
    if text.is_empty() {
        return (String::new(), HashMap::new());
    }

    let frequencies = build_frequency_map(text);
    let tree = build_huffman_tree(frequencies).expect("Failed to build Huffman tree");

    let mut codes = HashMap::new();
    generate_codes(&tree, String::new(), &mut codes);

    let encoded: String = text.chars().map(|ch| codes[&ch].as_str()).collect();

    (encoded, codes)
}

/// Decodes a Huffman-encoded string
///
/// # Arguments
///
/// * `encoded` - The binary string to decode
/// * `codes` - HashMap of character to binary code mappings
///
/// # Returns
///
/// The decoded original string
///
/// # Examples
///
/// ```
/// # use std::collections::HashMap;
/// # use the_algorithms_rust::compression::{huffman_encode, huffman_decode};
/// let text = "hello world";
/// let (encoded, codes) = huffman_encode(text);
/// let decoded = huffman_decode(&encoded, &codes);
/// assert_eq!(text, decoded);
/// ```
pub fn huffman_decode(encoded: &str, codes: &HashMap<char, String>) -> String {
    if encoded.is_empty() {
        return String::new();
    }

    // Reverse the code map for decoding
    let reverse_codes: HashMap<&str, char> = codes
        .iter()
        .map(|(ch, code)| (code.as_str(), *ch))
        .collect();

    let mut decoded = String::new();
    let mut current_code = String::new();

    for bit in encoded.chars() {
        current_code.push(bit);
        if let Some(&character) = reverse_codes.get(current_code.as_str()) {
            decoded.push(character);
            current_code.clear();
        }
    }

    decoded
}

/// Demonstrates Huffman encoding by processing a file and displaying detailed results
///
/// This function reads a file, encodes it using Huffman coding, and displays:
/// - Character code mappings
/// - Compression statistics
/// - Encoded output (with smart truncation for large files)
/// - Decoding verification
///
/// # Arguments
///
/// * `file_path` - Path to the file to encode
///
/// # Returns
///
/// Result indicating success or IO error
///
/// # Examples
///
/// ```ignore
/// // Note: This function is not re-exported in the public API
/// // Access it via: the_algorithms_rust::compression::huffman_encoding::demonstrate_huffman_from_file
/// use std::fs::File;
/// use std::io::Write;
///
/// // Create a test file
/// let mut file = File::create("test.txt").unwrap();
/// file.write_all(b"hello world").unwrap();
///
/// // Demonstrate Huffman encoding
/// // In your code, use the full path or import from huffman_encoding module
/// demonstrate_huffman_from_file("test.txt").unwrap();
/// ```
#[allow(dead_code)]
pub fn demonstrate_huffman_from_file(file_path: &str) -> std::io::Result<()> {
    // Read the file contents
    let text = fs::read_to_string(file_path)?;

    if text.is_empty() {
        println!("File is empty!");
        return Ok(());
    }

    // Encode using Huffman coding
    let (encoded, codes) = huffman_encode(&text);

    // Display the results
    println!("Huffman Coding of {file_path}: ");
    println!();

    // Show the code table
    println!("Character Codes:");
    println!("{:-<40}", "");
    let mut sorted_codes: Vec<_> = codes.iter().collect();
    sorted_codes.sort_by_key(|(ch, _)| *ch);

    for (ch, code) in sorted_codes {
        let display_char = if ch.is_whitespace() {
            format!("'{}' (space/whitespace)", ch.escape_default())
        } else {
            format!("'{ch}'")
        };
        println!("{display_char:20} -> {code}");
    }
    println!("{:-<40}", "");
    println!();

    // Show encoding statistics
    let original_bits = text.len() * 8; // Assuming 8-bit characters
    let compressed_bits = encoded.len();
    let compression_ratio = if original_bits > 0 {
        (1.0 - (compressed_bits as f64 / original_bits as f64)) * 100.0
    } else {
        0.0
    };

    println!("Statistics:");
    println!(
        "  Original size:    {} characters ({} bits)",
        text.len(),
        original_bits
    );
    println!("  Encoded size:     {compressed_bits} bits");
    println!("  Compression:      {compression_ratio:.2}%");
    println!();

    // Show the encoded output (limited to avoid overwhelming the terminal)
    println!("Encoded output:");
    if encoded.len() <= 500 {
        // Split into chunks of 50 for readability
        for (i, chunk) in encoded.as_bytes().chunks(50).enumerate() {
            print!("{:4}: ", i * 50);
            for &byte in chunk {
                print!("{}", byte as char);
            }
            println!();
        }
    } else {
        // Show first and last portions for very long outputs
        println!("  (showing first and last 200 bits)");
        print!("  Start: ");
        for &byte in &encoded.as_bytes()[..200] {
            print!("{}", byte as char);
        }
        println!();
        print!("  End:   ");
        for &byte in &encoded.as_bytes()[encoded.len() - 200..] {
            print!("{}", byte as char);
        }
        println!();
    }
    println!();

    // Verify decoding
    let decoded = huffman_decode(&encoded, &codes);
    if decoded == text {
        println!("✓ Decoding verification: SUCCESS");
    } else {
        println!("✗ Decoding verification: FAILED");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let (encoded, codes) = huffman_encode("");
        assert_eq!(encoded, "");
        assert!(codes.is_empty());
    }

    #[test]
    fn test_single_character() {
        let (encoded, codes) = huffman_encode("aaaa");
        assert_eq!(encoded, "0000");
        assert_eq!(codes.get(&'a'), Some(&"0".to_string()));
    }

    #[test]
    fn test_simple_string() {
        let text = "hello";
        let (encoded, codes) = huffman_encode(text);

        // Verify all characters have codes
        for ch in text.chars() {
            assert!(codes.contains_key(&ch), "Missing code for '{ch}'");
        }

        // Verify decoding returns original text
        let decoded = huffman_decode(&encoded, &codes);
        assert_eq!(decoded, text);
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let test_cases = vec![
            "a",
            "ab",
            "hello world",
            "the quick brown fox jumps over the lazy dog",
            "aaaaabbbbbcccccdddddeeeeefffffggggghhhhhiiiii",
        ];

        for text in test_cases {
            let (encoded, codes) = huffman_encode(text);
            let decoded = huffman_decode(&encoded, &codes);
            assert_eq!(decoded, text, "Failed roundtrip for: '{text}'");
        }
    }

    #[test]
    fn test_frequency_based_encoding() {
        // In "aaabbc", 'a' should have shorter code than 'b' or 'c'
        let (_, codes) = huffman_encode("aaabbc");
        let a_len = codes[&'a'].len();
        let b_len = codes[&'b'].len();
        let c_len = codes[&'c'].len();

        // 'a' appears most frequently, so should have shortest or equal code
        assert!(a_len <= b_len);
        assert!(a_len <= c_len);
    }

    #[test]
    fn test_compression_ratio() {
        let text = "aaaaaaaaaa"; // 10 'a's
        let (encoded, _) = huffman_encode(text);

        // Original: 10 chars * 8 bits = 80 bits (in UTF-8)
        // Huffman: 10 * 1 bit = 10 bits (single character gets code "0")
        assert_eq!(encoded.len(), 10);
        assert!(encoded.chars().all(|c| c == '0'));
    }

    #[test]
    fn test_all_unique_characters() {
        let text = "abcdefg";
        let (encoded, codes) = huffman_encode(text);

        // All characters should have codes
        assert_eq!(codes.len(), 7);

        // Verify roundtrip
        let decoded = huffman_decode(&encoded, &codes);
        assert_eq!(decoded, text);
    }

    #[test]
    fn test_build_frequency_map() {
        let frequencies = build_frequency_map("hello");
        assert_eq!(frequencies.get(&'h'), Some(&1));
        assert_eq!(frequencies.get(&'e'), Some(&1));
        assert_eq!(frequencies.get(&'l'), Some(&2));
        assert_eq!(frequencies.get(&'o'), Some(&1));
    }

    #[test]
    fn test_unicode_characters() {
        let text = "Hello, 世界! 🌍";
        let (encoded, codes) = huffman_encode(text);
        let decoded = huffman_decode(&encoded, &codes);
        assert_eq!(decoded, text);
    }

    #[test]
    fn test_demonstrate_huffman_from_file() {
        use std::fs::File;
        use std::io::Write;

        // Create a temporary test file
        let test_file = "/tmp/huffman_test.txt";
        let test_content = "The quick brown fox jumps over the lazy dog";

        {
            let mut file = File::create(test_file).unwrap();
            file.write_all(test_content.as_bytes()).unwrap();
        }

        // Test the demonstrate function
        let result = demonstrate_huffman_from_file(test_file);
        assert!(result.is_ok());
    }

    #[test]
    fn test_demonstrate_empty_file() {
        use std::fs::File;

        // Create an empty test file
        let test_file = "/tmp/huffman_empty.txt";
        File::create(test_file).unwrap();

        // Test with empty file
        let result = demonstrate_huffman_from_file(test_file);
        assert!(result.is_ok());
    }
}

/// Main function for command-line usage
///
/// Allows this file to be compiled as a standalone binary:
/// ```bash
/// rustc huffman_encoding.rs -o huffman
/// ./huffman input.txt
/// ```
#[cfg(not(test))]
#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Huffman Encoding - Lossless Data Compression");
        eprintln!();
        eprintln!("Usage: {} <file_path>", args[0]);
        eprintln!();
        eprintln!("Example:");
        eprintln!("  {} sample.txt", args[0]);
        eprintln!();
        eprintln!("This will encode the file and display:");
        eprintln!("  - Character code mappings");
        eprintln!("  - Compression statistics");
        eprintln!("  - Encoded binary output");
        eprintln!("  - Verification of successful decoding");
        std::process::exit(1);
    }

    let file_path = &args[1];

    match demonstrate_huffman_from_file(file_path) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error processing file '{file_path}': {e}");
            std::process::exit(1);
        }
    }
}
