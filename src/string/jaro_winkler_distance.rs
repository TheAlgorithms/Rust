// In computer science and statistics,
// the Jaro–Winkler distance is a string metric measuring an edit distance
// between two sequences.
// It is a variant proposed in 1990 by William E. Winkler
// of the Jaro distance metric (1989, Matthew A. Jaro).

pub fn jaro_winkler_distance(str1: &str, str2: &str) -> f64 {
    if str1.is_empty() || str2.is_empty() {
        return 0.0;
    }

    // Operate on character vectors throughout so that all indices and
    // lengths are character counts, never byte offsets. This keeps the
    // algorithm correct (not merely non-panicking) for arbitrary Unicode.
    let chars1: Vec<char> = str1.chars().collect();
    let chars2: Vec<char> = str2.chars().collect();

    fn get_matched_characters(s1: &[char], s2: &[char]) -> Vec<char> {
        let mut s2: Vec<char> = s2.to_vec();
        let mut matched: Vec<char> = Vec::new();
        let limit = std::cmp::min(s1.len(), s2.len()) / 2;
        for (i, &l) in s1.iter().enumerate() {
            let left = std::cmp::max(0, i as i32 - limit as i32) as usize;
            let right = std::cmp::min(i + limit + 1, s2.len());
            if s2[left..right].contains(&l) {
                matched.push(l);
                // Mark the first occurrence as used with a placeholder so it
                // can no longer be matched, while preserving the positions of
                // the remaining characters (mirrors the original space insertion).
                if let Some(pos) = s2.iter().position(|&c| c == l) {
                    s2[pos] = ' ';
                }
            }
        }
        matched
    }

    let matching_1 = get_matched_characters(&chars1, &chars2);
    let matching_2 = get_matched_characters(&chars2, &chars1);
    let match_count = matching_1.len();

    // transposition
    let transpositions = {
        let mut count = 0;
        for (c1, c2) in matching_1.iter().zip(matching_2.iter()) {
            if c1 != c2 {
                count += 1;
            }
        }
        count / 2
    };

    let jaro: f64 = {
        if match_count == 0 {
            return 0.0;
        }
        (1_f64 / 3_f64)
            * (match_count as f64 / chars1.len() as f64
                + match_count as f64 / chars2.len() as f64
                + (match_count - transpositions) as f64 / match_count as f64)
    };

    let mut prefix_len = 0.0;
    let bound = std::cmp::min(std::cmp::min(chars1.len(), chars2.len()), 4);
    for (c1, c2) in chars1[..bound].iter().zip(chars2[..bound].iter()) {
        if c1 == c2 {
            prefix_len += 1.0;
        } else {
            break;
        }
    }
    jaro + (0.1 * prefix_len * (1.0 - jaro))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jaro_winkler_distance() {
        let a = jaro_winkler_distance("hello", "world");
        assert_eq!(a, 0.4666666666666666);
        let a = jaro_winkler_distance("martha", "marhta");
        assert_eq!(a, 0.9611111111111111);
        let a = jaro_winkler_distance("martha", "marhat");
        assert_eq!(a, 0.9611111111111111);
        let a = jaro_winkler_distance("test", "test");
        assert_eq!(a, 1.0);
        let a = jaro_winkler_distance("test", "");
        assert_eq!(a, 0.0);
        let a = jaro_winkler_distance("hello world", "HeLLo W0rlD");
        assert_eq!(a, 0.6363636363636364);
    }

    #[test]
    fn test_jaro_winkler_distance_non_ascii() {
        // Regression test for issue #1047: multi-byte UTF-8 input must not
        // panic and must use character counts (not byte lengths).
        // "ab" and "céd" share no characters, so the distance is 0.0.
        let a = jaro_winkler_distance("ab", "céd");
        assert_eq!(a, 0.0);
    }

    #[test]
    fn test_jaro_winkler_distance_all_multibyte_identical() {
        // Two entirely non-ASCII strings that are identical. Characters,
        // not bytes, must be counted (each CJK char is 3 bytes).
        let a = jaro_winkler_distance("测试", "测试");
        assert_eq!(a, 1.0);

        // Identical combining-accent strings (each char is 2 bytes).
        let a = jaro_winkler_distance("áé", "áé");
        assert_eq!(a, 1.0);
    }

    #[test]
    fn test_jaro_winkler_distance_multibyte_partial() {
        // Non-identical multi-byte strings: character counting in the Jaro
        // denominator matters here (bytes would over-count the length).
        // "测试" and "测a": one of two characters matches.
        //   match_count = 1, transpositions = 0
        //   jaro = (1/3) * (1/2 + 1/2 + 1/1) = 2/3 ≈ 0.6666666666666666
        //   prefix = 1 ("测" matches), result = 2/3 + 0.1 * 1 * (1 - 2/3) = 0.7
        let a = jaro_winkler_distance("测试", "测a");
        assert_eq!(a, 0.7);
    }
}
