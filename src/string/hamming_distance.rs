pub fn hamming_distance(string_a: &str, string_b: &str) -> usize {
    if string_a.len() != string_b.len() {
        panic!("Strings must have the same length");
    }

    string_a
        .chars()
        .zip(string_b.chars())
        .filter(|(a, b)| a != b)
        .count()
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
        empty_inputs: ("", "", 0),
        length_1_inputs: ("a", "a", 0),
        same_strings: ("rust", "rust", 0),
        regular_input_0: ("karolin", "kathrin", 3),
        regular_input_1: ("kathrin", "kerstin", 4),
        regular_input_2: ("00000", "11111", 5),
        different_case: ("x", "X", 1),
    }

    #[test]
    #[should_panic]
    fn panic_when_inputs_are_of_different_length() {
        hamming_distance("0", "");
    }
}
