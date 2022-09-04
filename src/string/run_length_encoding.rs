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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(
            (run_length_encoding("".to_string())),
            "String is Empty!".to_string()
        )
    }

    #[test]
    fn identical_character() {
        assert_eq!(
            (run_length_encoding("aaaaaaaaaa".to_string())),
            "10a".to_string()
        )
    }
    #[test]
    fn continuous_character() {
        assert_eq!(
            (run_length_encoding("abcdefghijk".to_string())),
            "1a1b1c1d1e1f1g1h1i1j1k".to_string()
        )
    }

    #[test]
    fn random_character() {
        assert_eq!(
            (run_length_encoding("aaaaabbbcccccdddddddddd".to_string())),
            "5a3b5c10d".to_string()
        )
    }

    #[test]
    fn long_character() {
        assert_eq!(
            (run_length_encoding(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbcccccdddddddddd".to_string()
            )),
            "200a3b5c10d".to_string()
        )
    }
}
