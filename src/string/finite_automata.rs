fn get_next_state(pat: &String, m: usize, state: i32, x: i32) -> i32 {
    if state < m as i32 && x == pat.chars().nth(state as usize).unwrap() as i32 {
        return state + 1;
    }

    let mut i = 0;
    for ns in (1..=state).rev() {
        if pat.chars().nth((ns - 1) as usize).unwrap() as i32 == x {
            while i < ns - 1 {
                if pat.chars().nth(i as usize).unwrap()
                    != pat.chars().nth((state - ns + 1 + i) as usize).unwrap()
                {
                    break;
                }

                i += 1;
            }

            if i == ns - 1 {
                return ns;
            }
        }
    }
    0
}

fn compute_tf(pat: &String, m: usize, tf: &mut Vec<Vec<i32>>) {
    for state in 0..=m {
        for x in 0..256 {
            tf[state][x] = get_next_state(&pat, m, state as i32, x as i32);
        }
    }
}

/// Finite Automata algorithm for Pattern Searching
pub fn finite_automata(txt: String, pat: String) -> Vec<i32> {
    let mut result = Vec::<i32>::new();

    // This variable should be 1_112_064 because Rust `String` is utf8 compatible but the allocation size will be too high. Hence for the time being, it is kept to a reasonable 256 and hence it will only handle ASCII characters
    let d = 256 as usize;
    let m = pat.len();
    let n = txt.len();

    let mut tf: Vec<Vec<i32>> = (0..m + 1)
        .map(|_| {
            let mut vec = vec![];
            vec.resize(d, 0);
            vec
        })
        .collect();

    compute_tf(&pat, m, &mut tf);

    let mut state = 0;
    for i in 0..n {
        state = tf[state][txt.chars().nth(i).unwrap() as usize] as usize;
        if state == m {
            result.push(i as i32 - m as i32 + 1);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_automata_each_letter_matches() {
        let index = finite_automata("aaa".to_string(), "a".to_string());
        assert_eq!(index, vec![0, 1, 2]);
    }

    #[test]
    fn finite_automata_a_few_separate_matches() {
        let index = finite_automata("abababa".to_string(), "ab".to_string());
        assert_eq!(index, vec![0, 2, 4]);
    }

    #[test]
    fn finite_automata_one_match() {
        let index = finite_automata("ABC ABCDAB ABCDABCDABDE".to_string(), "ABCDABD".to_string());
        assert_eq!(index, vec![15]);
    }

    #[test]
    fn finite_automata_lots_of_matches() {
        let index = finite_automata("aaabaabaaaaa".to_string(), "aa".to_string());
        assert_eq!(index, vec![0, 1, 4, 7, 8, 9, 10]);
    }

    #[test]
    fn finite_automata_lots_of_intricate_matches() {
        let index = finite_automata("ababababa".to_string(), "aba".to_string());
        assert_eq!(index, vec![0, 2, 4, 6]);
    }

    #[test]
    fn finite_automata_not_found0() {
        let index = finite_automata("abcde".to_string(), "f".to_string());
        assert_eq!(index, vec![]);
    }

    #[test]
    fn finite_automata_not_found1() {
        let index = finite_automata("abcde".to_string(), "ac".to_string());
        assert_eq!(index, vec![]);
    }

    #[test]
    fn finite_automata_not_found2() {
        let index = finite_automata("ababab".to_string(), "bababa".to_string());
        assert_eq!(index, vec![]);
    }

    #[test]
    fn finite_automata_empty_string() {
        let index = finite_automata("".to_string(), "abcdef".to_string());
        assert_eq!(index, vec![]);
    }
}
