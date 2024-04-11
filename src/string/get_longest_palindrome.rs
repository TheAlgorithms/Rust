/*
Find the longest palindrome substring in the string
dp[i][j] show s[i]..s[j] a palindrome
so dp[i][j] = (s[i]==s[j]) && dp[i+1][j-1]
*/

pub fn get_longest_palindrome(s: &str) -> String {
    if s.is_empty() {
        return "".to_string();
    }
    let n = s.chars().count();
    let s: Vec<char> = s.chars().collect();
    let mut dp = vec![vec![true; n]; n];
    let mut res = (0, 0);

    for k in 1..n {
        for i in 0..(n - k) {
            if k == 1 {
                dp[i][i + k] = s[i] == s[i + 1];
            } else {
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
    use crate::string::get_longest_palindrome;

    #[test]
    fn empty() {
        assert_eq!(get_longest_palindrome(""), "");
    }

    #[test]
    fn longest_palindrome() {
        assert_eq!(get_longest_palindrome("abdbc"), "bdb");
    }

    #[test]
    fn unicode() {
        assert_eq!(get_longest_palindrome("常威天天打来福"), "天天");
    }
}
