pub fn shortest_palindrome(s: String) -> String {
    if s.is_empty() {
        return "String is Empty!".to_string();
    }

    let p_chars: Vec<char> = s.chars().collect();
    let mut suffix = vec![0; s.len()];
    for i in 1..s.len() {
        let mut j = suffix[i - 1];
        while j > 0 && p_chars[j] != p_chars[i] {
            j = suffix[j - 1];
        }
        suffix[i] = j + if p_chars[j] == p_chars[i] { 1 } else { 0 };
    }

    let mut dp = vec![0; s.len()];
    let mut s_chars: Vec<char> = s.chars().rev().collect();
    dp[0] = if p_chars[0] == s_chars[0] { 1 } else { 0 };
    for i in 1..s_chars.len() {
        let mut j = dp[i - 1];
        while j > 0 && s_chars[i] != p_chars[j] {
            j = suffix[j - 1];
        }
        dp[i] = j + if s_chars[i] == p_chars[j] { 1 } else { 0 };
    }

    s_chars.append(&mut p_chars[dp[s.len() - 1]..s.len()].to_vec());
    s_chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(shortest_palindrome("aacecaaa".to_string()), "aaacecaaa");
    }

    #[test]
    fn example_two() {
        assert_eq!(shortest_palindrome("abcd".to_string()), "dcbabcd");
    }
    #[test]
    fn example_three() {
        assert_eq!(shortest_palindrome("".to_string()), "String is Empty!");
    }
}
