/// Error type for Hamming distance calculation.
#[derive(Debug, PartialEq)]
pub enum HammingDistanceError {
    InputStringsHaveDifferentLength,
}

/// Calculates the Hamming distance between two strings.
///
/// The Hamming distance is defined as the number of positions at which the corresponding characters of the two strings are different.
pub fn hamming_distance(string_a: &str, string_b: &str) -> Result<usize, HammingDistanceError> {
    if string_a.len() != string_b.len() {
        return Err(HammingDistanceError::InputStringsHaveDifferentLength);
    }

    let distance = string_a
        .chars()
        .zip(string_b.chars())
        .filter(|(a, b)| a != b)
        .count();

    Ok(distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_hamming_distance {
        ($($name:ident: $tc:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (str_a, str_b, expected) = $tc;
                    assert_eq!(hamming_distance(str_a, str_b), expected);
                    assert_eq!(hamming_distance(str_b, str_a), expected);
                }
            )*
        }
    }

    test_hamming_distance! {
        empty_inputs: ("", "", Ok(0)),
        different_length: ("0", "", Err(HammingDistanceError::InputStringsHaveDifferentLength)),
        length_1_inputs_identical: ("a", "a", Ok(0)),
        length_1_inputs_different: ("a", "b", Ok(1)),
        same_strings: ("rust", "rust", Ok(0)),
        regular_input_0: ("karolin", "kathrin", Ok(3)),
        regular_input_1: ("kathrin", "kerstin", Ok(4)),
        regular_input_2: ("00000", "11111", Ok(5)),
        different_case: ("x", "X", Ok(1)),
        strings_with_no_common_chars: ("abcd", "wxyz", Ok(4)),
        long_strings_one_diff: (&"a".repeat(1000), &("a".repeat(999) + "b"), Ok(1)),
        long_strings_many_diffs: (&("a".repeat(500) + &"b".repeat(500)), &("b".repeat(500) + &"a".repeat(500)), Ok(1000)),
        strings_with_special_chars_identical: ("!@#$%^", "!@#$%^", Ok(0)),
        strings_with_special_chars_diff: ("!@#$%^", "&*()_+", Ok(6)),
    }
}
