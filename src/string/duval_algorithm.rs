// A string is called simple (or a Lyndon word), if it is strictly smaller than any of its own nontrivial suffixes.
// Duval (1983) developed an algorithm for finding the standard factorization that runs in linear time and constant space. Source: https://en.wikipedia.org/wiki/Lyndon_word
fn factorization_with_duval(s: Vec<char>) -> Vec<String> {
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
            factorization.push(s[i..i + j - k].iter().collect::<String>());
            i += j - k;
        }
    }

    factorization
}

pub fn duval_algorithm(s: &str) -> Vec<String> {
    return factorization_with_duval(s.chars().collect::<Vec<char>>());
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_duval_algorithm {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (text, expected) = $inputs;
                assert_eq!(duval_algorithm(text), expected);
            }
        )*
        }
    }

    test_duval_algorithm! {
        multiple: ("abcdabcdababc", ["abcd", "abcd", "ababc"]),
        all: ("aaa", ["a", "a", "a"]),
        single: ("ababb", ["ababb"]),
        unicode: ("അഅഅ", ["അ", "അ", "അ"]),
    }

    #[test]
    fn test_factorization_with_duval_multiple() {
        let text = "abcdabcdababc";
        assert_eq!(
            factorization_with_duval(text.chars().collect::<Vec<char>>()),
            ["abcd", "abcd", "ababc"]
        );
    }
}
