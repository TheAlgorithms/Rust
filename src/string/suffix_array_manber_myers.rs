pub fn generate_suffix_array_manber_myers(input: &str) -> Vec<usize> {
    if input.is_empty() {
        return Vec::new();
    }
    let n = input.len();
    let mut suffixes: Vec<(usize, &str)> = Vec::with_capacity(n);

    for (i, _suffix) in input.char_indices() {
        suffixes.push((i, &input[i..]));
    }

    suffixes.sort_by_key(|&(_, s)| s);

    let mut suffix_array: Vec<usize> = vec![0; n];
    let mut rank = vec![0; n];

    let mut cur_rank = 0;
    let mut prev_suffix = &suffixes[0].1;

    for (i, suffix) in suffixes.iter().enumerate() {
        if &suffix.1 != prev_suffix {
            cur_rank += 1;
            prev_suffix = &suffix.1;
        }
        rank[suffix.0] = cur_rank;
        suffix_array[i] = suffix.0;
    }

    let mut k = 1;
    let mut new_rank: Vec<usize> = vec![0; n];

    while k < n {
        suffix_array.sort_by_key(|&x| (rank[x], rank[(x + k) % n]));

        let mut cur_rank = 0;
        let mut prev = suffix_array[0];
        new_rank[prev] = cur_rank;

        for &suffix in suffix_array.iter().skip(1) {
            let next = suffix;
            if (rank[prev], rank[(prev + k) % n]) != (rank[next], rank[(next + k) % n]) {
                cur_rank += 1;
            }
            new_rank[next] = cur_rank;
            prev = next;
        }

        std::mem::swap(&mut rank, &mut new_rank);

        k <<= 1;
    }

    suffix_array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_array() {
        let input = "banana";
        let expected_result = vec![5, 3, 1, 0, 4, 2];
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        let expected_result: Vec<usize> = Vec::new();
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }

    #[test]
    fn test_single_character() {
        let input = "a";
        let expected_result = vec![0];
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }
    #[test]
    fn test_repeating_characters() {
        let input = "zzzzzz";
        let expected_result = vec![5, 4, 3, 2, 1, 0];
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }

    #[test]
    fn test_long_string() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let expected_result: Vec<usize> = (0..26).collect();
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }

    #[test]
    fn test_mix_of_characters() {
        let input = "abracadabra!";
        let expected_result = vec![11, 10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2];
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }

    #[test]
    fn test_whitespace_characters() {
        let input = " hello world ";
        let expected_result = vec![12, 0, 6, 11, 2, 1, 10, 3, 4, 5, 8, 9, 7];
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }
}
