// In computer science, a suffix array is a sorted array of all suffixes of a string.
// It is a data structure used in, among others, full-text indices, data-compression algorithms,
// and the field of bibliometrics. Source: https://en.wikipedia.org/wiki/Suffix_array

use std::cmp::Ordering;

#[derive(Clone)]
struct Suffix {
    index: usize,
    rank: (i32, i32),
}

impl Suffix {
    fn cmp(&self, b: &Self) -> Ordering {
        let a = self;
        let ((a1, a2), (b1, b2)) = (a.rank, b.rank);
        match a1.cmp(&b1) {
            Ordering::Equal => {
                if a2 < b2 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            o => o,
        }
    }
}

pub fn generate_suffix_array(txt: &str) -> Vec<usize> {
    let n = txt.len();
    let mut suffixes: Vec<Suffix> = vec![
        Suffix {
            index: 0,
            rank: (-1, -1)
        };
        n
    ];
    for (i, suf) in suffixes.iter_mut().enumerate() {
        suf.index = i;
        suf.rank.0 = (txt.chars().nth(i).expect("this should exist") as u32 - 'a' as u32) as i32;
        suf.rank.1 = if (i + 1) < n {
            (txt.chars().nth(i + 1).expect("this should exist") as u32 - 'a' as u32) as i32
        } else {
            -1
        }
    }
    suffixes.sort_by(|a, b| a.cmp(b));
    let mut ind = vec![0; n];
    let mut k = 4;
    while k < 2 * n {
        let mut rank = 0;
        let mut prev_rank = suffixes[0].rank.0;
        suffixes[0].rank.0 = rank;
        ind[suffixes[0].index] = 0;

        for i in 1..n {
            if suffixes[i].rank.0 == prev_rank && suffixes[i].rank.1 == suffixes[i - 1].rank.1 {
                prev_rank = suffixes[i].rank.0;
                suffixes[i].rank.0 = rank;
            } else {
                prev_rank = suffixes[i].rank.0;
                rank += 1;
                suffixes[i].rank.0 = rank;
            }
            ind[suffixes[i].index] = i;
        }
        for i in 0..n {
            let next_index = suffixes[i].index + (k / 2);
            suffixes[i].rank.1 = if next_index < n {
                suffixes[ind[next_index]].rank.0
            } else {
                -1
            }
        }
        suffixes.sort_by(|a, b| a.cmp(b));
        k *= 2;
    }
    let mut suffix_arr = Vec::new();
    for suf in suffixes {
        suffix_arr.push(suf.index);
    }
    suffix_arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_array() {
        let a = generate_suffix_array("banana");
        assert_eq!(a, vec![5, 3, 1, 0, 4, 2]);
    }
}
