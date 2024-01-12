//*
// Fn that checks if the slice is a lipogram
// Lipogram - sentence that exclude one or more letters from the alphabet
//
// if you only need one result use is_lipogram(str).0 for bool or use is_lipogram(str).1 for String
pub fn is_lipogram(lipogram_str: &str) -> (bool, String) {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut excluded_letters = vec![];

    if lipogram_str.is_empty() {
        return (false, String::from("The sentence is empty"));
    }

    for letter in alphabet.chars() {
        if !lipogram_str.to_lowercase().contains(letter) {
            excluded_letters.push(letter);
        }
    }

    if excluded_letters.len() != 0 {
        let excluded_letters_joined = excluded_letters.iter().map(|&c| c.to_string()).collect::<Vec<String>>().join(", ");
        let result = format!("{}{}", "The sentence is a lipogram that excludes the letters: ", excluded_letters_joined);
        (true, result)
    } else {
        (false, String::from("The sentence is not a lipogram"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lipogram_invalid1() {
        assert_eq!(is_lipogram("The quick brown fox jumps over the lazy dog"), (false, String::from("The sentence is not a lipogram")));
    }

    #[test]
    fn test_lipogram_invalid2() {
        assert_eq!(is_lipogram("Jackdaws love my big sphinx of quartz"), (false, String::from("The sentence is not a lipogram")));
        assert_eq!(is_lipogram("abcdefghijklmnopqrstuvwxyz"), (false, String::from("The sentence is not a lipogram")));
        assert_eq!(is_lipogram("Five quacking zephyrs jolt my wax bed"), (false, String::from("The sentence is not a lipogram")));
    }

    #[test]
    fn test_lipogram_invalid3() {
        assert_eq!(is_lipogram(""), (false, String::from("The sentence is empty")));
    }

    #[test]
    fn test_lipogram_valid1() {
        assert_eq!(is_lipogram("abcdefghijklmnopqrstuvwxy"), (true, String::from("The sentence is a lipogram that excludes the letters: z")));
    }

    #[test]
    fn test_lipogram_valid2() {
        assert_eq!(is_lipogram("The quick brown fox jumped over the lazy dog"), (true, String::from("The sentence is a lipogram that excludes the letters: s")));
        assert_eq!(is_lipogram("The brown fox jumped over the lazy dog with a brick"), (true, String::from("The sentence is a lipogram that excludes the letters: q, s")));
        assert_eq!(is_lipogram("The brown cat jumped over the lazy dog with a brick"), (true, String::from("The sentence is a lipogram that excludes the letters: f, q, s, x")));
    }
}
