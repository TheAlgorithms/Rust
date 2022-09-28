// In computer science and statistics,
// the Jaro–Winkler distance is a string metric measuring an edit distance
// between two sequences.
// It is a variant proposed in 1990 by William E. Winkler
// of the Jaro distance metric (1989, Matthew A. Jaro).

pub fn jaro_winkler_distance(str1: &str, str2: &str) -> f64 {
    if str1.is_empty() || str2.is_empty() {
        return 0.0;
    }
    fn get_matched_characters(s1: &str, s2: &str) -> String {
        let mut s2 = s2.to_string();
        let mut matched: Vec<char> = Vec::new();
        let limit = std::cmp::min(s1.len(), s2.len()) / 2;
        for (i, l) in s1.chars().enumerate() {
            let left = std::cmp::max(0, i as i32 - limit as i32) as usize;
            let right = std::cmp::min(i + limit + 1, s2.len());
            if s2[left..right].contains(l) {
                matched.push(l);
                let a = &s2[0..s2.find(l).expect("this exists")];
                let b = &s2[(s2.find(l).expect("this exists") + 1)..];
                s2 = format!("{a} {b}");
            }
        }
        matched.iter().collect::<String>()
    }

    let matching_1 = get_matched_characters(str1, str2);
    let matching_2 = get_matched_characters(str2, str1);
    let match_count = matching_1.len();

    // transposition
    let transpositions = {
        let mut count = 0;
        for (c1, c2) in matching_1.chars().zip(matching_2.chars()) {
            if c1 != c2 {
                count += 1;
            }
        }
        count / 2
    };

    let jaro: f64 = {
        if match_count == 0 {
            return 0.0;
        } else {
            (1_f64 / 3_f64)
                * (match_count as f64 / str1.len() as f64
                    + match_count as f64 / str2.len() as f64
                    + (match_count - transpositions) as f64 / match_count as f64)
        }
    };

    let mut prefix_len = 0.0;
    let bound = std::cmp::min(std::cmp::min(str1.len(), str2.len()), 4);
    for (c1, c2) in str1[..bound].chars().zip(str2[..bound].chars()) {
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
}
