/*
Find the longest palindrome substring in the string
Example:

Input: s = "abdbc"
Output: "bdb"
*/

pub fn get_longest_palindrome(s: &str) -> String {
    if s.is_empty() {
        return "".to_string();
    }
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    // dp indicates whether it is a palindrome
    let mut dp = vec![vec![true; n]; n];
    let mut res = (0, 0);

    for k in 1..n {
        for i in 0..(n - k) {
            if k == 1 {
                dp[i][i + k] = s[i] == s[i + 1];
            } else {
                dp[i][i + k] = (s[i] == s[i + k]) && dp[i + 1][i + k - 1];
            }

            // update palindrome length
            if dp[i][i + k] {
                res = (i, i + k);
            }
        }
    }
    s[res.0..=res.1].iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::get_longest_palindrome;
    macro_rules! test_get_longest_palindrome {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    use crate::string::is_palindrome;
                    let (s, expected) = $inputs;
                    assert!(is_palindrome(expected));
                    assert_eq!(get_longest_palindrome(s), expected);
                    assert_eq!(get_longest_palindrome(expected), expected);
                }
            )*
        }
    }
    test_get_longest_palindrome! {
        empty_input: ("", ""),
        basic_1: ("abdbc", "bdb"),
        basic_2: ("abyxycbabcyxy", "yxycbabcyxy"),
        unicode_1: ("常威天天打来福", "天天"),
    }
}
