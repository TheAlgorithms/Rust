//! This module contains the Smith-Waterman algorithm implementation for local sequence alignment.
//!
//! The Smith-Waterman algorithm is a dynamic programming algorithm used for determining
//! similar regions between two sequences (nucleotide or protein sequences). It is particularly
//! useful in bioinformatics for identifying optimal local alignments.
//!
//! # Algorithm Overview
//!
//! The algorithm works by:
//! 1. Creating a scoring matrix where each cell represents the maximum alignment score
//!    ending at that position
//! 2. Using match, mismatch, and gap penalties to calculate scores
//! 3. Allowing scores to reset to 0 (ensuring local rather than global alignment)
//! 4. Tracing back from the highest scoring position to reconstruct the alignment
//!
//! # Time Complexity
//!
//! O(m * n) where m and n are the lengths of the two sequences
//!
//! # Space Complexity
//!
//! O(m * n) for the scoring matrix
//!
//! # References
//!
//! - [Smith, T.F., Waterman, M.S. (1981). "Identification of Common Molecular Subsequences"](https://doi.org/10.1016/0022-2836(81)90087-5)
//! - [Wikipedia: Smith-Waterman algorithm](https://en.wikipedia.org/wiki/Smith%E2%80%93Waterman_algorithm)

use std::cmp::max;

/// Calculates the score for a character pair based on match, mismatch, or gap scoring.
///
/// # Arguments
///
/// * `source_char` - Character from the source sequence
/// * `target_char` - Character from the target sequence
/// * `match_score` - Score awarded for matching characters (typically positive)
/// * `mismatch_score` - Score penalty for mismatching characters (typically negative)
/// * `gap_score` - Score penalty for gaps (typically negative)
///
/// # Returns
///
/// The calculated score for the character pair
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::dynamic_programming::score_function;
///
/// let score = score_function('A', 'A', 1, -1, -2);
/// assert_eq!(score, 1); // Match
///
/// let score = score_function('A', 'C', 1, -1, -2);
/// assert_eq!(score, -1); // Mismatch
///
/// let score = score_function('-', 'A', 1, -1, -2);
/// assert_eq!(score, -2); // Gap
/// ```
pub fn score_function(
    source_char: char,
    target_char: char,
    match_score: i32,
    mismatch_score: i32,
    gap_score: i32,
) -> i32 {
    if source_char == '-' || target_char == '-' {
        gap_score
    } else if source_char == target_char {
        match_score
    } else {
        mismatch_score
    }
}

/// Performs the Smith-Waterman local sequence alignment algorithm.
///
/// This function creates a scoring matrix using dynamic programming to find the
/// optimal local alignment between two sequences. The algorithm is case-insensitive.
///
/// # Arguments
///
/// * `query` - The query sequence (e.g., DNA, protein)
/// * `subject` - The subject sequence to align against
/// * `match_score` - Score for matching characters (default: 1)
/// * `mismatch_score` - Penalty for mismatching characters (default: -1)
/// * `gap_score` - Penalty for gaps/indels (default: -2)
///
/// # Returns
///
/// A 2D vector representing the dynamic programming scoring matrix
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::dynamic_programming::smith_waterman;
///
/// let score_matrix = smith_waterman("ACAC", "CA", 1, -1, -2);
/// assert_eq!(score_matrix.len(), 5); // query length + 1
/// assert_eq!(score_matrix[0].len(), 3); // subject length + 1
/// ```
pub fn smith_waterman(
    query: &str,
    subject: &str,
    match_score: i32,
    mismatch_score: i32,
    gap_score: i32,
) -> Vec<Vec<i32>> {
    let query_upper: Vec<char> = query.to_uppercase().chars().collect();
    let subject_upper: Vec<char> = subject.to_uppercase().chars().collect();

    let m = query_upper.len();
    let n = subject_upper.len();

    // Initialize scoring matrix with zeros
    let mut score = vec![vec![0; n + 1]; m + 1];

    // Fill the scoring matrix using dynamic programming
    for i in 1..=m {
        for j in 1..=n {
            // Calculate score for match/mismatch
            let match_or_mismatch = score[i - 1][j - 1]
                + score_function(
                    query_upper[i - 1],
                    subject_upper[j - 1],
                    match_score,
                    mismatch_score,
                    gap_score,
                );

            // Calculate score for deletion (gap in subject)
            let delete = score[i - 1][j] + gap_score;

            // Calculate score for insertion (gap in query)
            let insert = score[i][j - 1] + gap_score;

            // Take maximum of all options, but never go below 0 (local alignment)
            score[i][j] = max(0, max(match_or_mismatch, max(delete, insert)));
        }
    }

    score
}

/// Performs traceback on the Smith-Waterman score matrix to reconstruct the optimal alignment.
///
/// This function starts from the highest scoring cell and traces back through the matrix
/// to reconstruct the aligned sequences. The traceback stops when a cell with score 0
/// is encountered.
///
/// # Arguments
///
/// * `score` - The score matrix from the Smith-Waterman algorithm
/// * `query` - Original query sequence used in alignment
/// * `subject` - Original subject sequence used in alignment
///
/// # Returns
///
/// A String containing the two aligned sequences separated by a newline,
/// or an empty string if no significant alignment is found
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::dynamic_programming::{smith_waterman, traceback};
///
/// let score_matrix = smith_waterman("ACAC", "CA", 1, -1, -2);
/// let alignment = traceback(&score_matrix, "ACAC", "CA");
/// assert_eq!(alignment, "CA\nCA");
/// ```
pub fn traceback(score: &[Vec<i32>], query: &str, subject: &str) -> String {
    let query_upper: Vec<char> = query.to_uppercase().chars().collect();
    let subject_upper: Vec<char> = subject.to_uppercase().chars().collect();

    // Find the cell with maximum score
    let mut max_value = i32::MIN;
    let (mut i_max, mut j_max) = (0, 0);

    for (i, row) in score.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value > max_value {
                max_value = value;
                i_max = i;
                j_max = j;
            }
        }
    }

    // If no significant alignment found, return empty string
    if i_max == 0 || j_max == 0 {
        return String::new();
    }

    // Traceback from the maximum scoring cell
    let (mut i, mut j) = (i_max, j_max);
    let mut align1 = String::new();
    let mut align2 = String::new();

    // Continue tracing back until we hit a cell with score 0
    while i > 0 && j > 0 && score[i][j] > 0 {
        let current_score = score[i][j];

        // Check if we came from diagonal (match/mismatch)
        if current_score
            == score[i - 1][j - 1]
                + score_function(query_upper[i - 1], subject_upper[j - 1], 1, -1, -2)
        {
            align1.insert(0, query_upper[i - 1]);
            align2.insert(0, subject_upper[j - 1]);
            i -= 1;
            j -= 1;
        }
        // Check if we came from above (deletion/gap in subject)
        else if current_score == score[i - 1][j] - 2 {
            align1.insert(0, query_upper[i - 1]);
            align2.insert(0, '-');
            i -= 1;
        }
        // Otherwise we came from left (insertion/gap in query)
        else {
            align1.insert(0, '-');
            align2.insert(0, subject_upper[j - 1]);
            j -= 1;
        }
    }

    format!("{align1}\n{align2}")
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! smith_waterman_tests {
        ($($name:ident: $test_cases:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (query, subject, match_score, mismatch_score, gap_score, expected) = $test_cases;
                    assert_eq!(smith_waterman(query, subject, match_score, mismatch_score, gap_score), expected);
                }
            )*
        }
    }

    macro_rules! traceback_tests {
        ($($name:ident: $test_cases:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (score, query, subject, expected) = $test_cases;
                    assert_eq!(traceback(&score, query, subject), expected);
                }
            )*
        }
    }

    smith_waterman_tests! {
        test_acac_ca: ("ACAC", "CA", 1, -1, -2, vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 1, 0],
            vec![0, 0, 2],
            vec![0, 1, 0],
        ]),
        test_agt_agt: ("AGT", "AGT", 1, -1, -2, vec![
            vec![0, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 2, 0],
            vec![0, 0, 0, 3],
        ]),
        test_agt_gta: ("AGT", "GTA", 1, -1, -2, vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 1],
            vec![0, 1, 0, 0],
            vec![0, 0, 2, 0],
        ]),
        test_agt_g: ("AGT", "G", 1, -1, -2, vec![
            vec![0, 0],
            vec![0, 0],
            vec![0, 1],
            vec![0, 0],
        ]),
        test_g_agt: ("G", "AGT", 1, -1, -2, vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 1, 0],
        ]),
        test_empty_query: ("", "CA", 1, -1, -2, vec![vec![0, 0, 0]]),
        test_empty_subject: ("ACAC", "", 1, -1, -2, vec![vec![0], vec![0], vec![0], vec![0], vec![0]]),
        test_both_empty: ("", "", 1, -1, -2, vec![vec![0]]),
    }

    traceback_tests! {
        test_traceback_acac_ca: (
            vec![
                vec![0, 0, 0],
                vec![0, 0, 1],
                vec![0, 1, 0],
                vec![0, 0, 2],
                vec![0, 1, 0],
            ],
            "ACAC",
            "CA",
            "CA\nCA",
        ),
        test_traceback_agt_agt: (
            vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 0, 2, 0],
                vec![0, 0, 0, 3],
            ],
            "AGT",
            "AGT",
            "AGT\nAGT",
        ),
        test_traceback_empty: (vec![vec![0, 0, 0]], "ACAC", "", ""),
    }

    #[test]
    fn test_score_function_match() {
        assert_eq!(score_function('A', 'A', 1, -1, -2), 1);
        assert_eq!(score_function('G', 'G', 2, -1, -1), 2);
    }

    #[test]
    fn test_score_function_mismatch() {
        assert_eq!(score_function('A', 'C', 1, -1, -2), -1);
        assert_eq!(score_function('G', 'T', 1, -2, -1), -2);
    }

    #[test]
    fn test_score_function_gap() {
        assert_eq!(score_function('-', 'A', 1, -1, -2), -2);
        assert_eq!(score_function('A', '-', 1, -1, -2), -2);
    }

    #[test]
    fn test_case_insensitive() {
        let result1 = smith_waterman("acac", "CA", 1, -1, -2);
        let result2 = smith_waterman("ACAC", "ca", 1, -1, -2);
        let result3 = smith_waterman("AcAc", "Ca", 1, -1, -2);

        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }
}
