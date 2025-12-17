/// Finds the minimum cuts needed for a palindrome partitioning of a string
///
/// Given a string s, partition s such that every substring of the partition is a palindrome.
/// This function returns the minimum number of cuts needed.
///
/// Time Complexity: O(n^2)
/// Space Complexity: O(n^2)
///
/// # Arguments
///
/// * `s` - The input string to partition
///
/// # Returns
///
/// The minimum number of cuts needed
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::dynamic_programming::minimum_palindrome_partitions;
///
/// assert_eq!(minimum_palindrome_partitions("aab"), 1);
/// assert_eq!(minimum_palindrome_partitions("aaa"), 0);
/// assert_eq!(minimum_palindrome_partitions("ababbbabbababa"), 3);
/// ```
///
/// # Algorithm Explanation
///
/// The algorithm uses dynamic programming with two key data structures:
/// - `cut[i]`: minimum cuts needed for substring from index 0 to i
/// - `is_palindromic[j][i]`: whether substring from index j to i is a palindrome
///
/// For each position i, we check all possible starting positions j to determine
/// if the substring s[j..=i] is a palindrome. If it is, we update the minimum
/// cut count accordingly.
///
/// Reference: <https://www.youtube.com/watch?v=_H8V5hJUGd0>
pub fn minimum_palindrome_partitions(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let length = chars.len();

    if length == 0 {
        return 0;
    }

    // cut[i] represents the minimum cuts needed for substring from 0 to i
    let mut cut = vec![0; length];

    // is_palindromic[j][i] represents whether substring from j to i is a palindrome
    let mut is_palindromic = vec![vec![false; length]; length];

    for i in 0..length {
        let mut mincut = i;

        for j in 0..=i {
            // Check if substring from j to i is a palindrome
            // A substring is a palindrome if:
            // 1. The characters at both ends match (chars[i] == chars[j])
            // 2. AND either:
            //    - The substring length is less than 2 (single char or two same chars)
            //    - OR the inner substring (j+1 to i-1) is also a palindrome
            if chars[i] == chars[j] && (i - j < 2 || is_palindromic[j + 1][i - 1]) {
                is_palindromic[j][i] = true;
                mincut = if j == 0 {
                    // If the entire substring from 0 to i is a palindrome, no cuts needed
                    0
                } else {
                    // Otherwise, take minimum of current mincut and (cuts up to j-1) + 1
                    mincut.min(cut[j - 1] + 1)
                };
            }
        }

        cut[i] = mincut;
    }

    cut[length - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        // "aab" -> "aa" | "b" = 1 cut
        assert_eq!(minimum_palindrome_partitions("aab"), 1);

        // "aaa" is already a palindrome = 0 cuts
        assert_eq!(minimum_palindrome_partitions("aaa"), 0);

        // Complex case
        assert_eq!(minimum_palindrome_partitions("ababbbabbababa"), 3);
    }

    #[test]
    fn test_edge_cases() {
        // Empty string
        assert_eq!(minimum_palindrome_partitions(""), 0);

        // Single character is always a palindrome
        assert_eq!(minimum_palindrome_partitions("a"), 0);

        // Two different characters need 1 cut
        assert_eq!(minimum_palindrome_partitions("ab"), 1);
    }

    #[test]
    fn test_palindromes() {
        // Already a palindrome
        assert_eq!(minimum_palindrome_partitions("racecar"), 0);
        assert_eq!(minimum_palindrome_partitions("noon"), 0);
        assert_eq!(minimum_palindrome_partitions("abba"), 0);
    }

    #[test]
    fn test_non_palindromes() {
        // All different characters need n-1 cuts
        assert_eq!(minimum_palindrome_partitions("abcde"), 4);

        // Two pairs need 1 cut
        assert_eq!(minimum_palindrome_partitions("aabb"), 1);
    }

    #[test]
    fn test_longer_strings() {
        assert_eq!(minimum_palindrome_partitions("aaabaa"), 1);
        assert_eq!(minimum_palindrome_partitions("abcbm"), 2);
    }
}
