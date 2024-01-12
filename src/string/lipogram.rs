use std::collections::HashSet;

//*
// Fn that returns the letters that are missing from the input slice
// and are present in the english alphabet
fn compute_missing(in_str: &str) -> String {
    let alphabet: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let letters_used: HashSet<char> = in_str
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    let difference_set: HashSet<char> = alphabet.difference(&letters_used).cloned().collect();

    difference_set
        .into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("")
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
    fn test_lipogram_invalid1() {
        assert_eq!(
            is_lipogram("The quick brown fox jumps over the lazy dog"),
            false
        );
        assert_eq!(
            compute_missing("The quick brown fox jumps over the lazy dog").len(),
            0
        );
    }

    #[test]
    fn test_lipogram_invalid2() {
        assert_eq!(is_lipogram("Jackdaws love my big sphinx of quartz"), false);
        assert_eq!(
            compute_missing("Jackdaws love my big sphinx of quartz").len(),
            0
        );
        assert_eq!(is_lipogram("abcdefghijklmnopqrstuvwxyz"), false);
        assert_eq!(
            compute_missing("Jackdaws love my big sphinx of quartz").len(),
            0
        );
        assert_eq!(is_lipogram("Five quacking zephyrs jolt my wax bed"), false);
        assert_eq!(
            compute_missing("Jackdaws love my big sphinx of quartz").len(),
            0
        );
    }

    #[test]
    fn test_lipogram_valid1() {
        assert_eq!(is_lipogram("abcdefghijklmnopqrstuvwxy"), true);
        assert_eq!(compute_missing("abcdefghijklmnopqrstuvwxy").len(), 1);
    }

    #[test]
    fn test_lipogram_valid2() {
        assert_eq!(
            is_lipogram("The quick brown fox jumped over the lazy dog"),
            true
        );
        assert_eq!(
            compute_missing("The quick brown fox jumped over the lazy dog").len(),
            1
        );
        assert_eq!(
            is_lipogram("The brown fox jumped over the lazy dog with a brick"),
            true
        );
        assert_eq!(
            compute_missing("The brown fox jumped over the lazy dog with a brick").len(),
            2
        );
        assert_eq!(
            is_lipogram("The brown cat jumped over the lazy dog with a brick"),
            true
        );
        assert_eq!(
            compute_missing("The brown cat jumped over the lazy dog with a brick").len(),
            4
        );
    }
}
