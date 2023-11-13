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

    #[test]
    fn empty_strings() {
        let result = hamming_distance("", "");
        assert_eq!(result, 0);
    }
    #[test]
    fn distance_zero() {
        let result = hamming_distance("rust", "rust");
        assert_eq!(result, 0);
    }
    #[test]
    fn distance_three() {
        let result = hamming_distance("karolin", "kathrin");
        assert_eq!(result, 3);
    }
    #[test]
    fn distance_four() {
        let result = hamming_distance("kathrin", "kerstin");
        assert_eq!(result, 4);
    }
    #[test]
    fn distance_five() {
        let result = hamming_distance("00000", "11111");
        assert_eq!(result, 5);
    }
    #[test]
    #[should_panic]
    fn panic_when_inputs_are_of_different_length() {
        hamming_distance("0", "");
    }
}
