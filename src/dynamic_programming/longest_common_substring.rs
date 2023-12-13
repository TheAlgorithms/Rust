// Longest common substring via Dynamic Programming
// longest_common_substring(a, b) returns the length of longest common substring between the strings a and b.
pub fn longest_common_substring(text1: &str, text2: &str) -> i32 {
    let m = text1.len();
    let n = text2.len();

    let t1 = text1.as_bytes();
    let t2 = text2.as_bytes();

    // BottomUp Tabulation
    let mut dp = vec![vec![0; n + 1]; m + 1];
    let mut ans = 0;
    for i in 1..=m {
        for j in 1..=n {
            if t1[i - 1] == t2[j - 1] {
                dp[i][j] = 1 + dp[i - 1][j - 1];
                ans = std::cmp::max(ans, dp[i][j]);
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_longest_common_substring {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (text1, text2, expected) = $inputs;
                assert_eq!(longest_common_substring(text1, text2), expected);
                assert_eq!(longest_common_substring(text2, text1), expected);
            }
        )*
        }
    }

    test_longest_common_substring! {
        empty_inputs: ("", "", 0),
        one_empty_input: ("", "a", 0),
        single_same_char_input: ("a", "a", 1),
        single_different_char_input: ("a", "b", 0),
        regular_input_0: ("abcdef", "bcd", 3),
        regular_input_1: ("abcdef", "xabded", 2),
        regular_input_2: ("GeeksforGeeks", "GeeksQuiz", 5),
        regular_input_3: ("abcdxyz", "xyzabcd", 4),
        regular_input_4: ("zxabcdezy", "yzabcdezx", 6),
        regular_input_5: ("OldSite:GeeksforGeeks.org", "NewSite:GeeksQuiz.com", 10),
        regular_input_6: ("aaaaaaaaaaaaa", "bbb", 0),
    }
}
