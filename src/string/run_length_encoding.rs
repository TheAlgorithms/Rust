pub fn run_length_encoding(target: String) -> String {
    if target.trim().is_empty() {
        return "String is Empty!".to_string();
    }
    let mut count: i32 = 0;
    let mut base_character: String = "".to_string();
    let mut encoded_target = String::new();

    for c in target.as_str().chars() {
        if base_character == *"" {
            base_character = c.to_string();
        }
        if c.to_string() == base_character {
            count += 1;
        } else {
            encoded_target.push_str(&count.to_string());
            count = 1;
            encoded_target.push_str(&base_character);
            base_character = c.to_string();
        }
    }
    encoded_target.push_str(&count.to_string());
    encoded_target.push_str(&base_character);

    encoded_target
}

pub fn run_length_decoding(target: String) -> String {
    if target.trim().is_empty() {
        return "String is Empty!".to_string();
    }

    let mut character_count: String = String::new();
    let mut decoded_target = String::new();

    for c in target.as_str().chars() {
        character_count.push(c);
        let is_numeric: bool = character_count.parse::<i32>().is_ok();

        if !is_numeric {
            let pop_char: char = character_count.pop().unwrap();
            decoded_target.push_str(
                &pop_char
                    .to_string()
                    .repeat(character_count.parse().unwrap()),
            );
            character_count = "".to_string();
        }
    }

    decoded_target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_empty() {
        assert_eq!(
            (run_length_encoding("".to_string())),
            "String is Empty!".to_string()
        )
    }

    #[test]
    fn encode_identical_character() {
        assert_eq!(
            (run_length_encoding("aaaaaaaaaa".to_string())),
            "10a".to_string()
        )
    }
    #[test]
    fn encode_continuous_character() {
        assert_eq!(
            (run_length_encoding("abcdefghijk".to_string())),
            "1a1b1c1d1e1f1g1h1i1j1k".to_string()
        )
    }

    #[test]
    fn encode_random_character() {
        assert_eq!(
            (run_length_encoding("aaaaabbbcccccdddddddddd".to_string())),
            "5a3b5c10d".to_string()
        )
    }

    #[test]
    fn encode_long_character() {
        assert_eq!(
            (run_length_encoding(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbcccccdddddddddd".to_string()
            )),
            "200a3b5c10d".to_string()
        )
    }
    #[test]
    fn decode_empty() {
        assert_eq!(
            (run_length_decoding("".to_string())),
            "String is Empty!".to_string()
        )
    }

    #[test]
    fn decode_identical_character() {
        assert_eq!(
            (run_length_decoding("10a".to_string())),
            "aaaaaaaaaa".to_string()
        )
    }
    #[test]
    fn decode_continuous_character() {
        assert_eq!(
            (run_length_decoding("1a1b1c1d1e1f1g1h1i1j1k".to_string())),
            "abcdefghijk".to_string()
        )
    }

    #[test]
    fn decode_random_character() {
        assert_eq!(
            (run_length_decoding("5a3b5c10d".to_string())),
            "aaaaabbbcccccdddddddddd".to_string()
        )
    }

    #[test]
    fn decode_long_character() {
        assert_eq!(
            (run_length_decoding(
                "200a3b5c10d".to_string()
            )),
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbcccccdddddddddd".to_string()
        )
    }
}
