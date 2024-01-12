use std::collections::HashSet;

//*
// Pangram - sentence that contains all the letters in the alphabet at least once
// Perfect Pangram - sentence that contains all the letters in the alphabet once (also named perfect heterogram)
#[derive(PartialEq, Debug)]
pub enum PangramStatus {
    NotPangram,
    Pangram,
    PerfectPangram,
}
//*
// Fn that checks if the slice is a pangram
//
// if you only need one result use is_pangram(str).0 for bool or use is_pangram(str).1 for PangramStatus
pub fn is_pangram(pangram_str: &str) -> (bool, PangramStatus) {
    let alphabet: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let letters_used: HashSet<char> = pangram_str
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    if letters_used != alphabet {
        return (false, PangramStatus::NotPangram);
    };

    if pangram_str.chars().filter(|c| c.is_alphabetic()).count() == 26 {
        (true, PangramStatus::PerfectPangram)
    } else {
        (true, PangramStatus::Pangram)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pangram_invalid1() {
        assert_eq!(
            is_pangram("This is not a pangram"),
            (false, PangramStatus::NotPangram)
        );
    }

    #[test]
    fn test_pangram_invalid2() {
        assert_eq!(
            is_pangram("today is a good day"),
            (false, PangramStatus::NotPangram)
        );
        assert_eq!(
            is_pangram(
                "this is almost a pangram but it does not have bcfghjkqwxy and the last letter"
            ),
            (false, PangramStatus::NotPangram)
        );
    }

    #[test]
    fn test_pangram_valid1() {
        assert_eq!(
            is_pangram("The quick brown fox jumps over the lazy dog"),
            (true, PangramStatus::Pangram)
        );
    }

    #[test]
    fn test_pangram_valid2() {
        assert_eq!(
            is_pangram("A mad boxer shot a quick, gloved jab to the jaw of his dizzy opponent"),
            (true, PangramStatus::Pangram)
        );
        assert_eq!(
            is_pangram("Amazingly few discotheques provide jukeboxes"),
            (true, PangramStatus::Pangram)
        );
        assert_eq!(
            is_pangram("How vexingly quick daft zebras jump"),
            (true, PangramStatus::Pangram)
        );
    }

    #[test]
    fn test_pangram_valid3() {
        assert_eq!(
            is_pangram("Mr. Jock, TV quiz PhD, bags few lynx"),
            (true, PangramStatus::PerfectPangram)
        );
    }
}
