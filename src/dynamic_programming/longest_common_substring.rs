// Longest common substring via Dynamic Programming
// longest_common_substring(a, b) returns the length of longest common substring between the strings a and b.
pub fn longest_common_substring(text1: String, text2: String) -> i32 {
    let m = text1.len();
    let n = text2.len();

    let t1 = text1.as_bytes();
    let t2 = text2.as_bytes();

    // BottomUp Tabulation
    let mut dp = vec![vec![0; n + 1]; m + 1];
    let mut ans = 0;
    for i in 1..=m {
        for j in 1..=n {
            if i == 0 || j == 0 {
                dp[i][j] = 0;
                continue;
            }
            if t1[i - 1] == t2[j - 1] {
                dp[i][j] = 1 + dp[i - 1][j - 1];
                ans = std::cmp::max(ans, dp[i][j]);
            }
        }
    }

    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            longest_common_substring(String::from(""), String::from("")),
            0
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            longest_common_substring(String::from("a"), String::from("")),
            0
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            longest_common_substring(String::from(""), String::from("a")),
            0
        );
    }
    #[test]
    fn test4() {
        assert_eq!(
            longest_common_substring(String::from("a"), String::from("a")),
            1
        );
    }
    #[test]
    fn test5() {
        assert_eq!(
            longest_common_substring(String::from("abcdef"), String::from("bcd")),
            3
        );
    }
    #[test]
    fn test6() {
        assert_eq!(
            longest_common_substring(String::from("abcdef"), String::from("xabded")),
            2
        );
    }
    #[test]
    fn test7() {
        assert_eq!(
            longest_common_substring(String::from("GeeksforGeeks"), String::from("GeeksQuiz")),
            5
        );
    }
    #[test]
    fn test8() {
        assert_eq!(
            longest_common_substring(String::from("abcdxyz"), String::from("xyzabcd")),
            4
        );
    }
    #[test]
    fn test9() {
        assert_eq!(
            longest_common_substring(String::from("zxabcdezy"), String::from("yzabcdezx")),
            6
        );
    }
    #[test]
    fn test10() {
        assert_eq!(
            longest_common_substring(
                String::from("OldSite:GeeksforGeeks.org"),
                String::from("NewSite:GeeksQuiz.com")
            ),
            10
        );
    }
}
