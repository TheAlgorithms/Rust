use std::collections::HashSet;

//*
// Fn that returns the letters that are missing from the input slice
// and are present in the english alphabet
fn compute_missing(in_str: &str) -> HashSet<char> {
    let alphabet: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let letters_used: HashSet<char> = in_str
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    alphabet.difference(&letters_used).cloned().collect()
}

//*
// Fn that checks if the slice is a lipogram.
// Lipogram - sentence in which a particular letter or group of letters is avoided
pub fn is_lipogram(lipogram_str: &str) -> bool {
    let unused_letters = compute_missing(lipogram_str);

    !unused_letters.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_lipogram_1() {
        assert!(!is_lipogram("The quick brown fox jumps over the lazy dog"));
        assert!(compute_missing("The quick brown fox jumps over the lazy dog").is_empty());
    }

    #[test]
    fn test_not_lipogram_2() {
        assert!(!is_lipogram("Jackdaws love my big sphinx of quartz"));
        assert!(compute_missing("Jackdaws love my big sphinx of quartz").is_empty());
        assert!(!is_lipogram("abcdefghijklmnopqrstuvwxyz"));
        assert!(compute_missing("abcdefghijklmnopqrstuvwxyz").is_empty());
        assert!(!is_lipogram("Five quacking zephyrs jolt my wax bed"));
        assert!(compute_missing("Five quacking zephyrs jolt my wax bed").is_empty());
    }

    #[test]
    fn test_valid_lipogram_1() {
        assert!(is_lipogram("abcdefghijklmnopqrstuvwxy"));
        assert_eq!(
            compute_missing("abcdefghijklmnopqrstuvwxy"),
            HashSet::from(['z'])
        );
    }

    #[test]
    fn test_valid_lipogram_2() {
        assert!(is_lipogram("The quick brown fox jumped over the lazy dog"));
        assert_eq!(
            compute_missing("The quick brown fox jumped over the lazy dog"),
            HashSet::from(['s'])
        );
        assert!(is_lipogram(
            "The brown fox jumped over the lazy dog with a brick"
        ));
        assert_eq!(
            compute_missing("The brown fox jumped over the lazy dog with a brick"),
            HashSet::from(['q', 's'])
        );
        assert!(is_lipogram(
            "The brown cat jumped over the lazy dog with a brick"
        ));
        assert_eq!(
            compute_missing("The brown cat jumped over the lazy dog with a brick"),
            HashSet::from(['f', 'q', 's', 'x'])
        );
    }
}
