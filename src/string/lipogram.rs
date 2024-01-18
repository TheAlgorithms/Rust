use std::collections::HashSet;

//*
// Fn that returns the letters that are missing from the input slice
// and are present in the english alphabet
///
/// ## Arguments
///
/// * `in_str` - the slice that will be checked for missing characters
///
/// ## Examples
///
/// ```
/// use the_algorithms_rust::string::compute_missing;
/// use std::collections::HashSet;
///
/// assert!(compute_missing("The quick brown fox jumps over the lazy dog").is_empty());
///
/// assert_eq!(
///    compute_missing("The brown cat jumped over the lazy dog with a brick"),
///    HashSet::from(['f', 'q', 's', 'x'])
/// );
/// ```
pub fn compute_missing(in_str: &str) -> HashSet<char> {
    let alphabet: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let letters_used: HashSet<char> = in_str
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    alphabet.difference(&letters_used).cloned().collect()
}

//*
// Fn that checks if the slice is a lipogram with specific missing letters.
// Lipogram - sentence in which a particular letter or group of letters is avoided
///
/// ## Arguments
///
/// * `lipogram_str` - the slice that will be checked if is a lipogram with specific missing letters
/// * `missing_chars` - the characters that has to be missing
///
/// ## Examples
///
/// ```
/// use the_algorithms_rust::string::is_lipogram;
/// use std::collections::HashSet;
///
/// assert!(
///    !is_lipogram("The quick brown fox jumps over the lazy dog",
///    &HashSet::from(['x'])
/// ));
///
/// assert!(
///    is_lipogram("The brown cat jumped over the lazy dog with a brick",
///    &HashSet::from(['f', 'q', 's', 'x'])
/// ));
///
/// assert!(
///    !is_lipogram("The quick brown fox jumped over the lazy dog",
///    &HashSet::from(['x'])
/// ));
/// ```
pub fn is_lipogram(lipogram_str: &str, missing_chars: &HashSet<char>) -> bool {
    let unused_letters = compute_missing(lipogram_str);

    missing_chars == &unused_letters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_lipogram() {
        let missing_chars = HashSet::from(['x']);
        assert!(!is_lipogram(
            "The quick brown fox jumps over the lazy dog",
            &missing_chars
        ));
        assert!(compute_missing("The quick brown fox jumps over the lazy dog").is_empty());

        let missing_chars = HashSet::from(['z']);
        assert!(!is_lipogram(
            "Jackdaws love my big sphinx of quartz",
            &missing_chars
        ));
        assert!(compute_missing("Jackdaws love my big sphinx of quartz").is_empty());

        let missing_chars = HashSet::from(['a']);
        assert!(!is_lipogram("abcdefghijklmnopqrstuvwxyz", &missing_chars));
        assert!(compute_missing("abcdefghijklmnopqrstuvwxyz").is_empty());

        let missing_chars = HashSet::from(['d']);
        assert!(!is_lipogram(
            "Five quacking zephyrs jolt my wax bed",
            &missing_chars
        ));
        assert!(compute_missing("Five quacking zephyrs jolt my wax bed").is_empty());

        let missing_chars = HashSet::from(['x']);
        let actual_missing_chars = HashSet::from(['s']);
        assert!(!is_lipogram(
            "The quick brown fox jumped over the lazy dog",
            &missing_chars
        ));
        assert_eq!(
            compute_missing("The quick brown fox jumped over the lazy dog"),
            actual_missing_chars
        );
    }

    #[test]
    fn test_valid_lipogram() {
        let missing_chars = HashSet::from(['z']);
        assert!(is_lipogram("abcdefghijklmnopqrstuvwxy", &missing_chars));
        assert_eq!(compute_missing("abcdefghijklmnopqrstuvwxy"), missing_chars);

        let missing_chars = HashSet::from(['s']);
        assert!(is_lipogram(
            "The quick brown fox jumped over the lazy dog",
            &missing_chars
        ));
        assert_eq!(
            compute_missing("The quick brown fox jumped over the lazy dog"),
            missing_chars
        );

        let missing_chars = HashSet::from(['q', 's']);
        assert!(is_lipogram(
            "The brown fox jumped over the lazy dog with a brick",
            &missing_chars
        ));
        assert_eq!(
            compute_missing("The brown fox jumped over the lazy dog with a brick"),
            missing_chars
        );

        let missing_chars = HashSet::from(['f', 'q', 's', 'x']);
        assert!(is_lipogram(
            "The brown cat jumped over the lazy dog with a brick",
            &missing_chars
        ));
        assert_eq!(
            compute_missing("The brown cat jumped over the lazy dog with a brick"),
            missing_chars
        );
    }
}
