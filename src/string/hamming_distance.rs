pub fn hamming_distance(string1: &str, string2: &str) -> usize {
    let mut distance = 0;
    let mut string1 = string1.chars();
    let mut string2 = string2.chars();

    loop {
        match (string1.next(), string2.next()) {
            (Some(char1), Some(char2)) if char1 != char2 => distance += 1,
            (Some(char1), Some(char2)) if char1 == char2 => continue,
            (None, Some(_)) | (Some(_), None) => panic!("Strings must have the same length"),
            (None, None) => break,
            _ => unreachable!(),
        }
    }
    distance
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
