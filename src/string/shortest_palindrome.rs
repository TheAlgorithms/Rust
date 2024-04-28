/*
The function shortest_palindrome expands the given string to shortest palindrome by adding a shortest prefix.
KMP. Source：https://www.scaler.com/topics/data-structures/kmp-algorithm/
Prefix Functions and KPM. Source：https://oi-wiki.org/string/kmp/
*/

pub fn shortest_palindrome(s: &str) -> String {
    if s.is_empty() {
        return "".to_string();
    }

    let p_chars: Vec<char> = s.chars().collect();
    let suffix = raw_suffix_function(&p_chars);

    let mut s_chars: Vec<char> = s.chars().rev().collect();
    // The prefix of the original string matches the suffix of the flipped string.
    let dp = invert_suffix_function(&p_chars, &s_chars, &suffix);

    s_chars.append(&mut p_chars[dp[p_chars.len() - 1]..p_chars.len()].to_vec());
    s_chars.iter().collect()
}

pub fn raw_suffix_function(p_chars: &[char]) -> Vec<usize> {
    let mut suffix = vec![0; p_chars.len()];
    for i in 1..p_chars.len() {
        let mut j = suffix[i - 1];
        while j > 0 && p_chars[j] != p_chars[i] {
            j = suffix[j - 1];
        }
        suffix[i] = j + if p_chars[j] == p_chars[i] { 1 } else { 0 };
    }
    suffix
}

pub fn invert_suffix_function(p_chars: &[char], s_chars: &[char], suffix: &[usize]) -> Vec<usize> {
    let mut dp = vec![0; p_chars.len()];
    dp[0] = if p_chars[0] == s_chars[0] { 1 } else { 0 };
    for i in 1..p_chars.len() {
        let mut j = dp[i - 1];
        while j > 0 && s_chars[i] != p_chars[j] {
            j = suffix[j - 1];
        }
        dp[i] = j + if s_chars[i] == p_chars[j] { 1 } else { 0 };
    }
    dp
}

#[cfg(test)]
mod tests {
    use crate::string::shortest_palindrome;
    macro_rules! test_shortest_palindrome {
    ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                use crate::string::is_palindrome;
                let (s, expected) = $inputs;
                assert!(is_palindrome(expected));
                assert_eq!(shortest_palindrome(s), expected);
                assert_eq!(shortest_palindrome(expected), expected);
            }
        )*
        }
    }
    test_shortest_palindrome! {
        empty: ("", ""),
        extend_left_1: ("aacecaaa", "aaacecaaa"),
        extend_left_2: ("abcd", "dcbabcd"),
        unicode_1: ("അ", "അ"),
        unicode_2: ("a牛", "牛a牛"),
    }
}
