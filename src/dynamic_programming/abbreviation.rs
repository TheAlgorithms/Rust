//! Abbreviation Problem Solution
//!
//! This module solves the abbreviation problem: determining if string `a` can be 
//! transformed into string `b` by capitalizing zero or more lowercase letters and 
//! deleting all remaining lowercase letters.

/// Determines if string `a` can be transformed into string `b` by:
/// 1. Capitalizing zero or more lowercase letters in `a`
/// 2. Deleting all remaining lowercase letters
///
/// The solution uses dynamic programming where `dp[i][j]` represents whether 
/// the first `i` characters of `a` can form the first `j` characters of `b`.
///
/// # Arguments
/// * `a` - The input string that may contain both uppercase and lowercase letters
/// * `b` - The target string containing only uppercase letters
///
/// # Returns
/// * A boolean indicating whether the transformation is possible
///
/// # Complexity
/// * Time complexity: O(n * m) where n is length of string a and m is length of string b
/// * Space complexity: O(n * m) for the DP table
///
/// # Examples
/// ```
/// use the_algorithms_rust::dynamic_programming::abbreviation;
/// 
/// assert_eq!(abbreviation("daBcd", "ABC"), true);
/// assert_eq!(abbreviation("dBcd", "ABC"), false);
/// ```
pub fn abbreviation(a: &str, b: &str) -> bool {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let n = a_chars.len();
    let m = b_chars.len();
    
    // dp[i][j] represents whether first i chars of a can form first j chars of b
    let mut dp = vec![vec![false; m + 1]; n + 1];
    
    // Base case: empty string a can form empty string b
    dp[0][0] = true;
    
    // Fill the first column: we can form empty b by deleting all lowercase letters
    for i in 0..n {
        if a_chars[i].is_lowercase() {
            dp[i + 1][0] = dp[i][0];
        }
    }
    
    for i in 0..n {
        for j in 0..=m {
            if dp[i][j] {
                // If we can match current position, check next characters
                if j < m && a_chars[i].to_ascii_uppercase() == b_chars[j] {
                    dp[i + 1][j + 1] = true;
                }
                
                // If current character in a is lowercase, we can delete it
                if a_chars[i].is_lowercase() {
                    dp[i + 1][j] = true;
                }
            }
        }
    }
    
    dp[n][m]
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! abbreviation_tests {
        ($($name:ident: ($a:expr, $b:expr) => $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(abbreviation($a, $b), $expected);
                }
            )*
        };
    }

    abbreviation_tests! {
        // Original test cases from the problem
        test_daBcd_ABC: ("daBcd", "ABC") => true,
        test_dBcd_ABC: ("dBcd", "ABC") => false,
        test_AbcE_ABE: ("AbcE", "ABE") => true,
        test_AbcE_ABC: ("AbcE", "ABC") => false,
        test_abcde_ABCDE: ("abcde", "ABCDE") => true,
        test_abcde_ABCD: ("abcde", "ABCD") => false,
        test_ABCDE_ABCDE: ("ABCDE", "ABCDE") => true,
        test_ABCDE_ABCD: ("ABCDE", "ABCD") => false,
        test_aBcDe_ABCDE: ("aBcDe", "ABCDE") => true,
        test_aBcDe_ABCD: ("aBcDe", "ABCD") => true,
        
        // Edge test cases
        test_empty_both: ("", "") => true,
        test_empty_a: ("", "ABC") => false,
        test_empty_b: ("abc", "") => true,
        test_only_lowercase: ("abc", "ABC") => true,
        test_only_uppercase: ("ABC", "ABC") => true,
        test_mismatched_uppercase: ("ABD", "ABC") => false,
        
        // Complex cases from HackerRank
        test_complex_1: ("LLZOSYAMQRMBTZXTQMQcKGLR", "LLZOSYAMBTZXMQKLR") => false,
        test_complex_2: ("MGYXKOVSMAHKOLAZZKWXKS", "MGXKOVSAHKOLZKKDP") => false,
        test_complex_3: ("bfBQZcnjXPMNWMZ", "BQZCNJXPMNWMZ") => true,
        test_AbcDE_ABDE: ("AbcDE", "ABDE") => true,
        test_AbcDE_AFDE: ("AbcDE", "AFDE") => false,
        test_ABCD_ABCD: ("ABCD", "ABCD") => true,
        test_abcdE_ABCDE: ("abcdE", "ABCDE") => true,
    }
}
