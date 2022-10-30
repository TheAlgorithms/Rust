// A string is called simple (or a Lyndon word), if it is strictly smaller than any of its own nontrivial suffixes.
// Duval (1983) developed an algorithm for finding the standard factorization that runs in linear time and constant space. Source: https://en.wikipedia.org/wiki/Lyndon_word
fn factorization_with_duval(s: &[u8]) -> Vec<String> {
    let n = s.len();
    let mut i = 0;
    let mut factorization: Vec<String> = Vec::new();

    while i < n {
        let mut j = i + 1;
        let mut k = i;

        while j < n && s[k] <= s[j] {
            if s[k] < s[j] {
                k = i;
            } else {
                k += 1;
            }
            j += 1;
        }

        while i <= k {
            factorization.push(String::from_utf8(s[i..i + j - k].to_vec()).unwrap());
            i += j - k;
        }
    }

    factorization
}

pub fn duval_algorithm(s: &str) -> Vec<String> {
    return factorization_with_duval(s.as_bytes());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_duval_multiple() {
        let text = "abcdabcdababc";
        assert_eq!(duval_algorithm(text), ["abcd", "abcd", "ababc"]);
    }

    #[test]
    fn test_duval_all() {
        let text = "aaa";
        assert_eq!(duval_algorithm(text), ["a", "a", "a"]);
    }

    #[test]
    fn test_duval_single() {
        let text = "ababb";
        assert_eq!(duval_algorithm(text), ["ababb"]);
    }

    #[test]
    fn test_factorization_with_duval_multiple() {
        let text = "abcdabcdababc";
        assert_eq!(
            factorization_with_duval(text.as_bytes()),
            ["abcd", "abcd", "ababc"]
        );
    }
}
