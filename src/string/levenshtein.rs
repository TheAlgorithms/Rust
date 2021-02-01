/// An algorithm for determining the difference between two strings using single
pub fn levenshtein(base: String, options: &Vec<String>) -> (usize, String) {
    let mut closest: (usize, String) = (0, base.clone());

    for vec_entry in options {
        let (mut entry, lower_base, lower_comparison) = (vec_entry.clone(), base.to_lowercase(), vec_entry.to_lowercase());
        let (a, b) = (lower_base.as_bytes(), lower_comparison.as_bytes());

        if a == b {
            closest = (0, String::from_utf8_lossy(b).to_string());
        } else {
            let mut len: usize = a.len();
            let (mut i, mut edits): (usize, usize) = (0, 0);
            if b.len() > len {
                len = b.len();
            }
            while i < len || i < len {
                if i >= a.len() || i >= b.len() {
                    edits += 1;
                } else if a[i] != b[i] {
                    edits += 1
                }
                i += 1;
            };

            if closest.0 > edits {
                closest = (edits, entry);
            }
        };
    }

    closest
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lev() {
        let result = super::levenshtein("Hey".to_string(), &vec!["Hi".to_string(), "hey".to_string(), "hello".to_string(), "helo".to_string()]);
        assert_eq!(result.0, 0)
    }
}
