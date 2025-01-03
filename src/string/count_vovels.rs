/// Counts the number of vowels in a given string.
/// Vowels are defined as 'a', 'e', 'i', 'o', 'u' (both uppercase and lowercase).
pub fn count_vowels(input: &str) -> usize {
    input.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_count_vowels {
    ($($name:ident: $tc:expr,)*) => {
      $(
        #[test]
        fn $name() {
          let (input, expected) = $tc;
          assert_eq!(count_vowels(input), expected);
        }
      )*
    }
  }

    test_count_vowels! {
      empty_string: ("", 0),
      no_vowels: ("ghfdryfs", 0),
      all_vowels_lowercase: ("aeiou", 5),
      all_vowels_uppercase: ("AEIOU", 5),
      mixed_vowels: ("aEIoU", 5),
      hello_world: ("Hello, World!", 3),
      long_string: (&"a".repeat(1000), 1000),
      string_with_special_chars: ("!@%^&+aei*()_o#$u", 5),
    }
}
