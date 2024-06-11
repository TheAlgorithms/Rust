pub fn run_length_encoding(target: &str) -> String {
    if target.trim().is_empty() {
        return "".to_string();
    }
    let mut count: i32 = 0;
    let mut base_character: String = "".to_string();
    let mut encoded_target = String::new();

    for c in target.chars() {
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

pub fn run_length_decoding(target: &str) -> String {
    if target.trim().is_empty() {
        return "".to_string();
    }
    let mut character_count: String = String::new();
    let mut decoded_target = String::new();

    for c in target.chars() {
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

    macro_rules! test_run_length {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (raw_str, encoded) = $test_case;
                    assert_eq!(run_length_encoding(raw_str), encoded);
                    assert_eq!(run_length_decoding(encoded), raw_str);
                }
            )*
        };
    }

    test_run_length! {
        empty_input: ("", ""),
        repeated_char: ("aaaaaaaaaa", "10a"),
        no_repeated: ("abcdefghijk", "1a1b1c1d1e1f1g1h1i1j1k"),
        regular_input: ("aaaaabbbcccccdddddddddd", "5a3b5c10d"),
        two_blocks_with_same_char: ("aaabbaaaa", "3a2b4a"),
        long_input: ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbcccccdddddddddd", "200a3b5c10d"),
    }
}
