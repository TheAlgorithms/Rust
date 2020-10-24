/// Rabin-Karp algorithm for pattern searching
pub fn rabin_karp(txt: String, pat: String, prime_number: i64) -> Vec<usize> {
    let mut res = Vec::<usize>::new();

    // Rust strings are utf8 compatible and hence they can store all the 1,112,064 characters and not just the 256 ASCII characters
    // Note: even if it is set to 256, result will have no effect
    let d = 1_112_064 as i64;

    let mut p = 0 as i64;
    let mut t = 0 as i64;
    let mut h = 1;

    let m = pat.len();
    let n = txt.len();

    if m != 0 && n != 0 {
        for _i in 0..m - 1 {
            h = (h * d) % prime_number;
        }

        for i in 0..m {
            p = ((d * p + pat.chars().nth(i).unwrap() as i64) % prime_number).into();
            t = ((d * t + txt.chars().nth(i).unwrap() as i64) % prime_number).into();
        }

        for i in 0..=(n - m) {
            if p == t {
                for j in 0..m {
                    if txt.chars().nth(i + j).unwrap() != pat.chars().nth(j).unwrap() {
                        break;
                    }

                    if j == m - 1 {
                        res.push(i);
                    }
                }
            }

            if i < n - m {
                t = (d * (t - txt.chars().nth(i).unwrap() as i64 * h)
                    + (txt.chars().nth(i + m).unwrap() as i64))
                    % prime_number;

                if t < 0 {
                    t = t + prime_number;
                }
            }
        }
    } else if n != 0 && m == 0 {
        res.push(0);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rabin_karp_each_letter_matches() {
        let index = rabin_karp("aaa".to_string(), "a".to_string(), 101);
        assert_eq!(index, vec![0, 1, 2]);
    }

    #[test]
    fn rabin_karp_a_few_separate_matches() {
        let index = rabin_karp("abababa".to_string(), "ab".to_string(), 101);
        assert_eq!(index, vec![0, 2, 4]);
    }

    #[test]
    fn rabin_karp_one_match() {
        let index = rabin_karp(
            "ABC ABCDAB ABCDABCDABDE".to_string(),
            "ABCDABD".to_string(),
            101,
        );
        assert_eq!(index, vec![15]);
    }

    #[test]
    fn rabin_karp_lots_of_matches() {
        let index = rabin_karp("aaabaabaaaaa".to_string(), "aa".to_string(), 101);
        assert_eq!(index, vec![0, 1, 4, 7, 8, 9, 10]);
    }

    #[test]
    fn rabin_karp_lots_of_intricate_matches() {
        let index = rabin_karp("ababababa".to_string(), "aba".to_string(), 101);
        assert_eq!(index, vec![0, 2, 4, 6]);
    }

    #[test]
    fn rabin_karp_not_found0() {
        let index = rabin_karp("abcde".to_string(), "f".to_string(), 101);
        assert_eq!(index, vec![]);
    }

    #[test]
    fn rabin_karp_not_found1() {
        let index = rabin_karp("abcde".to_string(), "ac".to_string(), 101);
        assert_eq!(index, vec![]);
    }

    #[test]
    fn rabin_karp_not_found2() {
        let index = rabin_karp("ababab".to_string(), "bababa".to_string(), 101);
        assert_eq!(index, vec![]);
    }

    #[test]
    fn rabin_karp_empty_string() {
        let index = rabin_karp("".to_string(), "abcdef".to_string(), 101);
        assert_eq!(index, vec![]);
    }
}
