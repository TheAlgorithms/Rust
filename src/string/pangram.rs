//*
// Fn that checks if the slice is a pangram
// Pangram - sentence that contains all the letters in the alphabet at least once
// Perfect Pangram - sentence that contains all the letters in the alphabet once (also named perfect heterogram)
//
// if you only need one result use is_pangram(str).0 for bool or use is_pangram(str).1 for &str
pub fn is_pangram(pangram_str: &str) -> (bool, &str) {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    if pangram_str.is_empty() {
        return (false, "The sentence is empty");
    }

    for letter in alphabet.chars() {
        if !pangram_str.to_lowercase().contains(letter) {
            return (false, "The sentence is not a pangram");
        }
    }

    if pangram_str.chars().filter(|c| c.is_alphabetic()).count() == 26 {
        (true, "The sentence is a perfect pangram")
    } else {
        (true, "The sentence is a pangram")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panagram_invalid1() {
        assert_eq!(
            is_pangram("This is not a pangram"),
            (false, "The sentence is not a pangram")
        );
    }

    #[test]
    fn test_panagram_invalid2() {
        assert_eq!(
            is_pangram("today is a good day"),
            (false, "The sentence is not a pangram")
        );
        assert_eq!(
            is_pangram(
                "this is almost a pangram but it does not have bcfghjkqwxy and the last letter"
            ),
            (false, "The sentence is not a pangram")
        );
    }

    #[test]
    fn test_panagram_empty() {
        assert_eq!(is_pangram(""), (false, "The sentence is empty"));
    }

    #[test]
    fn test_panagram_valid1() {
        assert_eq!(
            is_pangram("The quick brown fox jumps over the lazy dog"),
            (true, "The sentence is a pangram")
        );
    }

    #[test]
    fn test_panagram_valid2() {
        assert_eq!(
            is_pangram("A mad boxer shot a quick, gloved jab to the jaw of his dizzy opponent"),
            (true, "The sentence is a pangram")
        );
        assert_eq!(
            is_pangram("Amazingly few discotheques provide jukeboxes"),
            (true, "The sentence is a pangram")
        );
        assert_eq!(
            is_pangram("How vexingly quick daft zebras jump"),
            (true, "The sentence is a pangram")
        );
    }

    #[test]
    fn test_panagram_valid3() {
        assert_eq!(
            is_pangram("Mr. Jock, TV quiz PhD, bags few lynx"),
            (true, "The sentence is a perfect pangram")
        );
    }
}
