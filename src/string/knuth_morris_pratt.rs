pub fn knuth_morris_pratt(st: &str, pat: &str) -> Vec<usize> {
    if st.is_empty() || pat.is_empty() {
        return vec![];
    }

    let string = st.chars().collect::<Vec<char>>();
    let pattern = pat.chars().collect::<Vec<char>>();

    // build the partial match table
    let mut partial = vec![0];
    for i in 1..pattern.len() {
        let mut j = partial[i - 1];
        while j > 0 && pattern[j] != pattern[i] {
            j = partial[j - 1];
        }
        partial.push(if pattern[j] == pattern[i] { j + 1 } else { j });
    }

    // and read 'string' to find 'pattern'
    let mut ret = vec![];
    let mut j = 0;

    for (i, &c) in string.iter().enumerate() {
        while j > 0 && c != pattern[j] {
            j = partial[j - 1];
        }
        if c == pattern[j] {
            j += 1;
        }
        if j == pattern.len() {
            ret.push(i + 1 - j);
            j = partial[j - 1];
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_knuth_morris_pratt {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, pattern, expected) = $inputs;
                    let index = knuth_morris_pratt(input, pattern);
                    assert_eq!(index, expected);
                }
            )*
        }
    }

    test_knuth_morris_pratt! {
        each_letter_matches: ("aaa", "a", vec![0, 1, 2]),
        a_few_seperate_matches: ("abababa", "ab", vec![0, 2, 4]),
        unicode: ("അഅഅ", "അ", vec![0, 1, 2]),
        one_match: ("ABC ABCDAB ABCDABCDABDE",  "ABCDABD", vec![15]),
        lots_of_matches: ("aaabaabaaaaa",  "aa", vec![0, 1, 4, 7, 8, 9, 10]),
        lots_of_intricate_matches: ("ababababa", "aba", vec![0, 2, 4, 6]),
        not_found0: ("abcde", "f", vec![]),
        not_found1: ("abcde", "ac", vec![]),
        not_found2: ("ababab", "bababa", vec![]),
        empty_string: ("", "abcdef", vec![]),
    }
}
