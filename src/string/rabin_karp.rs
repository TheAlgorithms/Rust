const MODULUS: u16 = 101;
const BASE: u16 = 256;

pub fn rabin_karp(target: &str, pattern: &str) -> Vec<usize> {
    // Quick exit
    if target.is_empty() || pattern.is_empty() || pattern.len() > target.len() {
        return vec![];
    }

    let pattern_hash = hash(pattern);

    // Pre-calculate BASE^(n-1)
    let mut pow_rem: u16 = 1;
    for _ in 0..pattern.len() - 1 {
        pow_rem *= BASE;
        pow_rem %= MODULUS;
    }

    let mut rolling_hash = 0;
    let mut ret = vec![];
    for i in 0..=target.len() - pattern.len() {
        rolling_hash = if i == 0 {
            hash(&target[0..pattern.len()])
        } else {
            recalculate_hash(target, i - 1, i + pattern.len() - 1, rolling_hash, pow_rem)
        };
        if rolling_hash == pattern_hash && pattern[..] == target[i..i + pattern.len()] {
            ret.push(i);
        }
    }
    ret
}

// hash(s) is defined as BASE^(n-1) * s_0 + BASE^(n-2) * s_1 + ... + BASE^0 * s_(n-1)
fn hash(s: &str) -> u16 {
    let mut res: u16 = 0;
    for &c in s.as_bytes().iter() {
        res = (res * BASE % MODULUS + c as u16) % MODULUS;
    }
    res
}

// new_hash = (old_hash - BASE^(n-1) * s_(i-n)) * BASE + s_i
fn recalculate_hash(
    s: &str,
    old_index: usize,
    new_index: usize,
    old_hash: u16,
    pow_rem: u16,
) -> u16 {
    let mut new_hash = old_hash;
    let (old_ch, new_ch) = (
        s.as_bytes()[old_index] as u16,
        s.as_bytes()[new_index] as u16,
    );
    new_hash = (new_hash + MODULUS - pow_rem * old_ch % MODULUS) % MODULUS;
    new_hash = (new_hash * BASE + new_ch) % MODULUS;
    new_hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hi_hash() {
        let hash_result = hash("hi");
        assert_eq!(hash_result, 65);
    }

    #[test]
    fn abr_hash() {
        let hash_result = hash("abr");
        assert_eq!(hash_result, 4);
    }

    #[test]
    fn bra_hash() {
        let hash_result = hash("bra");
        assert_eq!(hash_result, 30);
    }

    // Attribution to @pgimalac for his tests from Knuth-Morris-Pratt
    #[test]
    fn each_letter_matches() {
        let index = rabin_karp("aaa", "a");
        assert_eq!(index, vec![0, 1, 2]);
    }

    #[test]
    fn a_few_separate_matches() {
        let index = rabin_karp("abababa", "ab");
        assert_eq!(index, vec![0, 2, 4]);
    }

    #[test]
    fn one_match() {
        let index = rabin_karp("ABC ABCDAB ABCDABCDABDE", "ABCDABD");
        assert_eq!(index, vec![15]);
    }

    #[test]
    fn lots_of_matches() {
        let index = rabin_karp("aaabaabaaaaa", "aa");
        assert_eq!(index, vec![0, 1, 4, 7, 8, 9, 10]);
    }

    #[test]
    fn lots_of_intricate_matches() {
        let index = rabin_karp("ababababa", "aba");
        assert_eq!(index, vec![0, 2, 4, 6]);
    }

    #[test]
    fn not_found0() {
        let index = rabin_karp("abcde", "f");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn not_found1() {
        let index = rabin_karp("abcde", "ac");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn not_found2() {
        let index = rabin_karp("ababab", "bababa");
        assert_eq!(index, vec![]);
    }

    #[test]
    fn empty_string() {
        let index = rabin_karp("", "abcdef");
        assert_eq!(index, vec![]);
    }
}
