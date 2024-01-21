use std::collections::HashSet;

/// Function that returns the letters that are missing from the input slice  
/// and are present in the English alphabet  
///
/// ## Arguments
///
/// * `in_str` - the slice that will be checked for missing characters
///
fn compute_missing(in_str: &str) -> HashSet<char> {
    let alphabet: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let letters_used: HashSet<char> = in_str
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    alphabet.difference(&letters_used).cloned().collect()
}

/// Function that checks if the slice is a lipogram with specific missing letters.  
/// Lipogram - sentence in which a particular letter or group of letters is avoided
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
    if !missing_chars.iter().all(|&c| c.is_lowercase()) {
        panic!("missing_chars should be all lowercase.")
    }

    missing_chars == &compute_missing(lipogram_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! test_lipogram {
    ($($name:ident: $inputs:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (in_str, missing_chars, other_chars) = $inputs;
            assert_ne!(missing_chars, other_chars);
            assert_eq!(compute_missing(in_str), missing_chars);
            assert!(is_lipogram(in_str, &missing_chars));
            assert!(!is_lipogram(in_str, &other_chars));
        }
    )*
    }
}

    test_lipogram! {
        lipogram1: ("The quick brown fox jumps over the lazy dog", HashSet::from([]), HashSet::from(['a', 'b'])),
        lipogram2: ("Jackdaws love my big sphinx of quartz", HashSet::from([]), HashSet::from(['x'])),
        lipogram3: ("abcdefghijklmnopqrstuvwxyz", HashSet::from([]), HashSet::from(['x', 'y', 'z'])),
        lipogram4: ("Five quacking zephyrs jolt my wax bed", HashSet::from([]), HashSet::from(['a'])),
        lipogram5: ("The quick brown fox jumped over the lazy dog", HashSet::from(['s']), HashSet::from([])),
        lipogram6: ("abcdefghijklmnopqrstuvwxy", HashSet::from(['z']), HashSet::from(['y', 'z'])),
        lipogram7: ("The brown fox jumped over the lazy dog with a brick", HashSet::from(['q', 's']), HashSet::from(['b'])),
        lipogram8: ("ABCdefghijklmnopqrstuvwx", HashSet::from(['y', 'z']), HashSet::from(['a', 'b'])),
    }

    #[test]
    #[should_panic]
    fn test_is_lipogram_panics_when_missing_chars_are_upper_case() {
        is_lipogram("abcdefghijklmnopqrstuvwx", &HashSet::from(['y', 'Z']));
    }
}
