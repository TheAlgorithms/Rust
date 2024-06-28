pub fn lz77_encode(input: &str) -> Vec<(usize, usize, char)> {
    let mut tokens = Vec::new();
    let mut index = 0;

    while index < input.len() {
        let mut best_match = (0, 0);
        let candidate = &input[index..input.len() - 1];

        for i in 0..index {
            let search_box = &input[i..index];
            let match_length = candidate
                .chars()
                .zip(search_box.chars())
                .take_while(|&(a, b)| a == b)
                .count();

            if match_length > best_match.1 {
                best_match = (index - i, match_length);
            }
        }

        if best_match.1 > 0 {
            tokens.push((
                best_match.0,
                best_match.1,
                input.chars().nth(index + best_match.1).unwrap(),
            ));
            index += best_match.1 + 1;
        } else {
            tokens.push((0, 0, input.chars().nth(index).unwrap()));
            index += 1;
        }
    }
    tokens
}

pub fn lz77_decode(tokens: Vec<(usize, usize, char)>) -> String {
    let mut result = String::new();
    for token in tokens {
        if token.0 != 0 {
            let start = result.len() - token.0;
            let length = token.1;
            let substring: String = result.chars().skip(start).take(length).collect();
            result += &substring;
        };
        result.push(token.2);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lz77_encode() {
        let res = lz77_encode("");
        assert_eq!(res, []);

        let res = lz77_encode("A");
        let expected: Vec<(usize, usize, char)> = vec![(0, 0, 'A')];
        assert_eq!(res, expected);

        let res = lz77_encode("AA");
        let expected: Vec<(usize, usize, char)> = vec![(0, 0, 'A'), (0, 0, 'A')];
        assert_eq!(res, expected);

        let res = lz77_encode("AAAABBBCCDAA");
        let expected: Vec<(usize, usize, char)> = vec![
            (0, 0, 'A'),
            (1, 1, 'A'),
            (3, 1, 'B'),
            (1, 1, 'B'),
            (0, 0, 'C'),
            (1, 1, 'D'),
            (10, 1, 'A'),
        ];
        assert_eq!(res, expected);

        let res = lz77_encode("Rust-Trends");
        let expected: Vec<(usize, usize, char)> = vec![
            (0, 0, 'R'),
            (0, 0, 'u'),
            (0, 0, 's'),
            (0, 0, 't'),
            (0, 0, '-'),
            (0, 0, 'T'),
            (0, 0, 'r'),
            (0, 0, 'e'),
            (0, 0, 'n'),
            (0, 0, 'd'),
            (0, 0, 's'),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_lz77_decode() {
        let res = lz77_decode(vec![]);
        assert_eq!(res, "");
        let res = lz77_decode(vec![(0, 0, 'A')]);
        assert_eq!(res, "A");
        let res = lz77_decode(vec![(0, 0, 'A'), (0, 0, 'A')]);
        assert_eq!(res, "AA");
        let res = lz77_decode(vec![
            (0, 0, 'A'),
            (1, 1, 'A'),
            (3, 1, 'B'),
            (1, 1, 'B'),
            (0, 0, 'C'),
            (1, 1, 'D'),
            (10, 1, 'A'),
        ]);
        assert_eq!(res, "AAAABBBCCDAA");
        let res = lz77_decode(vec![
            (0, 0, 'R'),
            (0, 0, 'u'),
            (0, 0, 's'),
            (0, 0, 't'),
            (0, 0, '-'),
            (0, 0, 'T'),
            (0, 0, 'r'),
            (0, 0, 'e'),
            (0, 0, 'n'),
            (0, 0, 'd'),
            (0, 0, 's'),
        ]);
        assert_eq!(res, "Rust-Trends");
    }
}
