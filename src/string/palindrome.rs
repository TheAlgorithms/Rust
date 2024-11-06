//! A module for checking if a given string is a palindrome.

/// Checks if the given string is a palindrome.
///
/// A palindrome is a sequence that reads the same backward as forward.
/// This function ignores non-alphanumeric characters and is case-insensitive.
///
/// # Arguments
///
/// * `s` - A string slice that represents the input to be checked.
///
/// # Returns
///
/// * `true` if the string is a palindrome; otherwise, `false`.
pub fn is_palindrome(s: &str) -> bool {
    let mut chars = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase());

    while let (Some(c1), Some(c2)) = (chars.next(), chars.next_back()) {
        if c1 != c2 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! palindrome_tests {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $inputs;
                    assert_eq!(is_palindrome(input), expected);
                }
            )*
        }
    }

    palindrome_tests! {
        odd_palindrome: ("madam", true),
        even_palindrome: ("deified", true),
        single_character_palindrome: ("x", true),
        single_word_palindrome: ("eye", true),
        case_insensitive_palindrome: ("RaceCar", true),
        mixed_case_and_punctuation_palindrome: ("A man, a plan, a canal, Panama!", true),
        mixed_case_and_space_palindrome: ("No 'x' in Nixon", true),
        empty_string: ("", true),
        pompeii_palindrome: ("Roma-Olima-Milo-Amor", true),
        napoleon_palindrome: ("Able was I ere I saw Elba", true),
        john_taylor_palindrome: ("Lewd did I live, & evil I did dwel", true),
        well_know_english_palindrome: ("Never odd or even", true),
        palindromic_phrase: ("Rats live on no evil star", true),
        names_palindrome: ("Hannah", true),
        prime_minister_of_cambodia: ("Lon Nol", true),
        japanese_novelist_and_manga_writer: ("Nisio Isin", true),
        actor: ("Robert Trebor", true),
        rock_vocalist: ("Ola Salo", true),
        pokemon_species: ("Girafarig", true),
        lychrel_num_56: ("121", true),
        universal_palindrome_date: ("02/02/2020", true),
        french_palindrome: ("une Slave valse nu", true),
        finnish_palindrome: ("saippuakivikauppias", true),
        non_palindrome_simple: ("hello", false),
        non_palindrome_with_punctuation: ("hello!", false),
        non_palindrome_mixed_case: ("Hello, World", false),
    }
}
