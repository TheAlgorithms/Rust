/*
Find the longest palindrome substring in the string（non-unique solution）
Source：https://leetcode.cn/problems/longest-palindromic-substring/description/
Example:

Input: s = "abdbc"
Output: "bdb"
*/

pub fn get_length_of_the_palindrome_with_max_length(s: &str) -> String {
    if s.is_empty() {
        return "".to_string();
    }
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    let mut dp = vec![vec![true; n]; n];
    // res is the index of the longest palindrome
    let mut res = (0, 0);

    // form filling strategy: fill in the form by
    // use k instead of palindrome length
    for k in 1..n {
        for i in 0..(n - k) {
            if k == 1 {
                // strings of 2 equal characters are palindromes
                dp[i][i + k] = s[i] == s[i + 1];
            } else {
                // a string that is equal on both sides and has a palindrome in the middle is also a palindrome
                dp[i][i + k] = (s[i] == s[i + k]) && dp[i + 1][i + k - 1];
            }

            if dp[i][i + k] {
                res = (i, i + k);
            }
        }
    }
    s[res.0..=res.1].iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::get_length_of_the_palindrome_with_max_length;
    macro_rules! test_get_length_of_the_palindrome_with_max_length {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    use crate::string::is_palindrome;
                    let (s, expected) = $inputs;
                    assert!(is_palindrome(expected));
                    assert_eq!(get_length_of_the_palindrome_with_max_length(s), expected);
                    assert_eq!(get_length_of_the_palindrome_with_max_length(expected), expected);
                }
            )*
        }
    }
    test_get_length_of_the_palindrome_with_max_length! {
        empty_input: ("", ""),
        basic_1: ("abdbc", "bdb"),
        basic_2: ("abyxycbabcyxy", "yxycbabcyxy"),
        // Theoretically it is possible to return either aa or bb,
        // there are multiple palindromes of the same length, here only the rightmost one is returned
        basic_3: ("aabb", "bb"),
        unicode_1: ("常威天天打来福", "天天"),
    }
}
