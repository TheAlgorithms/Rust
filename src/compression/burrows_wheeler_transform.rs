//! Burrows-Wheeler Transform
//!
//! The Burrows-Wheeler transform (BWT, also called block-sorting compression)
//! rearranges a character string into runs of similar characters. This is useful
//! for compression, since it tends to be easy to compress a string that has runs
//! of repeated characters by techniques such as move-to-front transform and
//! run-length encoding. More importantly, the transformation is reversible,
//! without needing to store any additional data except the position of the first
//! original character. The BWT is thus a "free" method of improving the efficiency
//! of text compression algorithms, costing only some extra computation.
//!
//! More info: <https://en.wikipedia.org/wiki/Burrows%E2%80%93Wheeler_transform>

/// Result of the Burrows-Wheeler transform containing the transformed string
/// and the index of the original string in the sorted rotations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BwtResult {
    /// The BWT-transformed string
    pub bwt_string: String,
    /// The index of the original string in the sorted rotations (0-based)
    pub idx_original_string: usize,
}

/// Generates all rotations of a string.
///
/// # Arguments
///
/// * `s` - The string to rotate
///
/// # Returns
///
/// A vector containing all rotations of the input string
///
/// # Examples
///
/// ```
/// # use the_algorithms_rust::compression::all_rotations;
/// let rotations = all_rotations("^BANANA|");
/// assert_eq!(rotations.len(), 8);
/// assert_eq!(rotations[0], "^BANANA|");
/// assert_eq!(rotations[1], "BANANA|^");
/// ```
pub fn all_rotations(s: &str) -> Vec<String> {
    (0..s.len())
        .map(|i| format!("{}{}", &s[i..], &s[..i]))
        .collect()
}

/// Performs the Burrows-Wheeler transform on a string.
///
/// # Arguments
///
/// * `s` - The string to transform (must not be empty)
///
/// # Returns
///
/// A `BwtResult` containing the transformed string and the index of the original string
///
/// # Panics
///
/// Panics if the input string is empty
///
/// # Examples
///
/// ```
/// # use the_algorithms_rust::compression::bwt_transform;
/// let result = bwt_transform("^BANANA");
/// assert_eq!(result.bwt_string, "BNN^AAA");
/// assert_eq!(result.idx_original_string, 6);
///
/// let result = bwt_transform("panamabanana");
/// assert_eq!(result.bwt_string, "mnpbnnaaaaaa");
/// assert_eq!(result.idx_original_string, 11);
/// ```
pub fn bwt_transform(s: &str) -> BwtResult {
    assert!(!s.is_empty(), "Input string must not be empty");

    let mut rotations = all_rotations(s);
    rotations.sort();

    // Find the index of the original string in sorted rotations
    let idx_original_string = rotations
        .iter()
        .position(|r| r == s)
        .expect("Original string must be in rotations");

    // Build BWT string from last character of each rotation
    let bwt_string: String = rotations
        .iter()
        .map(|r| r.chars().last().unwrap())
        .collect();

    BwtResult {
        bwt_string,
        idx_original_string,
    }
}

/// Reverses the Burrows-Wheeler transform to recover the original string.
///
/// # Arguments
///
/// * `bwt_string` - The BWT-transformed string
/// * `idx_original_string` - The 0-based index of the original string in sorted rotations
///
/// # Returns
///
/// The original string before BWT transformation
///
/// # Panics
///
/// * If `bwt_string` is empty
/// * If `idx_original_string` is out of bounds (>= length of `bwt_string`)
///
/// # Examples
///
/// ```
/// # use the_algorithms_rust::compression::reverse_bwt;
/// assert_eq!(reverse_bwt("BNN^AAA", 6), "^BANANA");
/// assert_eq!(reverse_bwt("aaaadss_c__aa", 3), "a_asa_da_casa");
/// assert_eq!(reverse_bwt("mnpbnnaaaaaa", 11), "panamabanana");
/// ```
pub fn reverse_bwt(bwt_string: &str, idx_original_string: usize) -> String {
    assert!(!bwt_string.is_empty(), "BWT string must not be empty");
    assert!(
        idx_original_string < bwt_string.len(),
        "Index must be less than BWT string length"
    );

    let len = bwt_string.len();
    let bwt_chars: Vec<char> = bwt_string.chars().collect();
    let mut ordered_rotations: Vec<String> = vec![String::new(); len];

    // Iteratively prepend characters and sort to reconstruct rotations
    for _ in 0..len {
        for i in 0..len {
            ordered_rotations[i] = format!("{}{}", bwt_chars[i], ordered_rotations[i]);
        }
        ordered_rotations.sort();
    }

    ordered_rotations[idx_original_string].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_rotations_banana() {
        let rotations = all_rotations("^BANANA|");
        assert_eq!(rotations.len(), 8);
        assert_eq!(
            rotations,
            vec![
                "^BANANA|", "BANANA|^", "ANANA|^B", "NANA|^BA", "ANA|^BAN", "NA|^BANA", "A|^BANAN",
                "|^BANANA"
            ]
        );
    }

    #[test]
    fn test_all_rotations_casa() {
        let rotations = all_rotations("a_asa_da_casa");
        assert_eq!(rotations.len(), 13);
        assert_eq!(rotations[0], "a_asa_da_casa");
        assert_eq!(rotations[1], "_asa_da_casaa");
        assert_eq!(rotations[12], "aa_asa_da_cas");
    }

    #[test]
    fn test_all_rotations_panama() {
        let rotations = all_rotations("panamabanana");
        assert_eq!(rotations.len(), 12);
        assert_eq!(rotations[0], "panamabanana");
        assert_eq!(rotations[11], "apanamabanan");
    }

    #[test]
    fn test_bwt_transform_banana() {
        let result = bwt_transform("^BANANA");
        assert_eq!(result.bwt_string, "BNN^AAA");
        assert_eq!(result.idx_original_string, 6);
    }

    #[test]
    fn test_bwt_transform_casa() {
        let result = bwt_transform("a_asa_da_casa");
        assert_eq!(result.bwt_string, "aaaadss_c__aa");
        assert_eq!(result.idx_original_string, 3);
    }

    #[test]
    fn test_bwt_transform_panama() {
        let result = bwt_transform("panamabanana");
        assert_eq!(result.bwt_string, "mnpbnnaaaaaa");
        assert_eq!(result.idx_original_string, 11);
    }

    #[test]
    #[should_panic(expected = "Input string must not be empty")]
    fn test_bwt_transform_empty() {
        bwt_transform("");
    }

    #[test]
    fn test_reverse_bwt_banana() {
        let original = reverse_bwt("BNN^AAA", 6);
        assert_eq!(original, "^BANANA");
    }

    #[test]
    fn test_reverse_bwt_casa() {
        let original = reverse_bwt("aaaadss_c__aa", 3);
        assert_eq!(original, "a_asa_da_casa");
    }

    #[test]
    fn test_reverse_bwt_panama() {
        let original = reverse_bwt("mnpbnnaaaaaa", 11);
        assert_eq!(original, "panamabanana");
    }

    #[test]
    #[should_panic(expected = "BWT string must not be empty")]
    fn test_reverse_bwt_empty_string() {
        reverse_bwt("", 0);
    }

    #[test]
    #[should_panic(expected = "Index must be less than BWT string length")]
    fn test_reverse_bwt_index_too_high() {
        reverse_bwt("mnpbnnaaaaaa", 12);
    }

    #[test]
    fn test_bwt_roundtrip() {
        // Test that transform -> reverse gives back original string
        let test_strings = vec![
            "^BANANA",
            "a_asa_da_casa",
            "panamabanana",
            "ABRACADABRA",
            "SIX.MIXED.PIXIES.SIFT.SIXTY.PIXIE.DUST.BOXES",
        ];

        for s in test_strings {
            let result = bwt_transform(s);
            let recovered = reverse_bwt(&result.bwt_string, result.idx_original_string);
            assert_eq!(recovered, s, "Roundtrip failed for '{s}'");
        }
    }

    #[test]
    fn test_single_character() {
        let result = bwt_transform("A");
        assert_eq!(result.bwt_string, "A");
        assert_eq!(result.idx_original_string, 0);

        let recovered = reverse_bwt(&result.bwt_string, result.idx_original_string);
        assert_eq!(recovered, "A");
    }

    #[test]
    fn test_repeated_characters() {
        let result = bwt_transform("AAAA");
        assert_eq!(result.bwt_string, "AAAA");

        let recovered = reverse_bwt(&result.bwt_string, result.idx_original_string);
        assert_eq!(recovered, "AAAA");
    }
}
