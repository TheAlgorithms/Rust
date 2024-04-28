/*
Given two strings s and t, determine whether they are isomorphic.
The two strings are isomorphic if the characters in s can be replaced by some mapping relation to get t
*/
use std::collections::HashMap;

pub fn is_isomorphic(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let sv: Vec<char> = s.chars().collect();
    let tv: Vec<char> = t.chars().collect();
    let mut sr1 = String::new();
    let mut map1 = HashMap::new();
    for i in 0..sv.len() {
        if let Some(x) = map1.get(&sv[i]) {
            sr1.push(*x);
        } else {
            map1.insert(sv[i], tv[i]);
            sr1.push(tv[i]);
        }
    }
    let mut sr2 = String::new();
    let mut map2 = HashMap::new();
    for i in 0..sv.len() {
        if let Some(x) = map2.get(&tv[i]) {
            sr2.push(*x);
        } else {
            map2.insert(tv[i], sv[i]);
            sr2.push(sv[i]);
        }
    }
    sr1 == t && sr2 == s
}

#[cfg(test)]
mod tests {
    use super::is_isomorphic;

    #[test]
    fn test_is_isomorphic1() {
        assert_eq!(is_isomorphic("egg", "add"), true);
    }
    #[test]
    fn test_is_isomorphic2() {
        assert_eq!(is_isomorphic("egg", "adc"), false);
    }
    #[test]
    fn test_unicode_is_isomorphic() {
        assert_eq!(is_isomorphic("天苍苍", "野茫茫"), true);
    }
    #[test]
    fn test_empty_is_isomorphic() {
        assert_eq!(is_isomorphic("", ""), true);
    }
}
