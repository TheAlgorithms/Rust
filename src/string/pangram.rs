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
pub fn is_pangram(pangram_str: &str) -> PangramStatus {
    let alphabet: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let letters_used: HashSet<char> = pangram_str
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    if letters_used != alphabet {
        return PangramStatus::NotPangram;
    };

    if pangram_str.chars().filter(|c| c.is_alphabetic()).count() == alphabet.len() {
        PangramStatus::PerfectPangram
    } else {
        PangramStatus::Pangram
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_pangram_1() {
        assert_eq!(
            is_pangram("This is not a pangram"),
            PangramStatus::NotPangram
        );
    }

    #[test]
    fn test_not_pangram_2() {
        assert_eq!(is_pangram("today is a good day"), PangramStatus::NotPangram);
        assert_eq!(
            is_pangram(
                "this is almost a pangram but it does not have bcfghjkqwxy and the last letter"
            ),
            PangramStatus::NotPangram
        );
    }

    #[test]
    fn test_valid_pangram_1() {
        assert_eq!(
            is_pangram("The quick brown fox jumps over the lazy dog"),
            PangramStatus::Pangram
        );
    }

    #[test]
    fn test_valid_pangram_2() {
        assert_eq!(
            is_pangram("A mad boxer shot a quick, gloved jab to the jaw of his dizzy opponent"),
            PangramStatus::Pangram
        );
        assert_eq!(
            is_pangram("Amazingly few discotheques provide jukeboxes"),
            PangramStatus::Pangram
        );
        assert_eq!(
            is_pangram("How vexingly quick daft zebras jump"),
            PangramStatus::Pangram
        );
    }

    #[test]
    fn test_valid_perfect_pangram() {
        assert_eq!(
            is_pangram("Mr. Jock, TV quiz PhD, bags few lynx"),
            PangramStatus::PerfectPangram
        );
    }
}
