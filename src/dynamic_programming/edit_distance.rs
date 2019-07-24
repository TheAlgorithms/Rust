//! Compute the edit distance between two strings

use std::cmp::min;

/// edit_distance(str_a, str_b) returns the edit distance between the two
/// strings This edit distance is defined as being 1 point per insertion,
/// substitution, or deletion which must be made to make the strings equal.
///
/// This function iterates over the bytes in the string, so it may not behave
/// entirely as expected for non-ASCII strings.
pub fn edit_distance(str_a: &str, str_b: &str) -> u32 {
    // distances[i][j] = distance between a[..i] and b[..j]
    let mut distances = vec![vec![0; str_b.len() + 1]; str_a.len() + 1];
    // Initialize cases in which one string is empty
    for j in 0..=str_b.len() {
        distances[0][j] = j as u32;
    }
    for (i, item) in distances.iter_mut().enumerate() {
        item[0] = i as u32;
    }
    for i in 1..=str_a.len() {
        for j in 1..=str_b.len() {
            distances[i][j] = min(distances[i - 1][j] + 1, distances[i][j - 1] + 1);
            if str_a.as_bytes()[i - 1] == str_b.as_bytes()[j - 1] {
                distances[i][j] = min(distances[i][j], distances[i - 1][j - 1]);
            } else {
                distances[i][j] = min(distances[i][j], distances[i - 1][j - 1] + 1);
            }
        }
    }
    distances[str_a.len()][str_b.len()]
}

#[cfg(test)]
mod tests {
    use super::edit_distance;

    #[test]
    fn equal_strings() {
        assert_eq!(0, edit_distance("Hello, world!", "Hello, world!"));
        assert_eq!(0, edit_distance("Test_Case_#1", "Test_Case_#1"));
    }

    #[test]
    fn one_edit_difference() {
        assert_eq!(1, edit_distance("Hello, world!", "Hell, world!"));
        assert_eq!(1, edit_distance("Test_Case_#1", "Test_Case_#2"));
        assert_eq!(1, edit_distance("Test_Case_#1", "Test_Case_#10"));
    }

    #[test]
    fn several_differences() {
        assert_eq!(2, edit_distance("My Cat", "My Case"));
        assert_eq!(7, edit_distance("Hello, world!", "Goodbye, world!"));
        assert_eq!(6, edit_distance("Test_Case_#3", "Case #3"));
    }
}
