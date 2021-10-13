pub fn is_subsequence(s: String, t: String) -> bool {
    let m = s.len();
    let n = t.len();
    let mut i = 0;
    let mut j = 0;

    let s = s.as_bytes();
    let t = t.as_bytes();

    while i < m && j < n {
        if s[i] == t[j] {
            i += 1;
        }

        j += 1;
    }

    if i == m { true } else { false }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_subsequence(String::from("abc"), String::from("ahbgdc")), true);
        assert_eq!(is_subsequence(String::from("axc"), String::from("ahbgdc")), false);
    }
}
