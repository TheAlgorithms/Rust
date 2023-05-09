use std::cmp::min;

/// The Levenshtein distance (or edit distance) between 2 strings.\
/// This edit distance is defined as being 1 point per insertion, substitution, or deletion which must be made to make the strings equal.
/// This function iterates over the bytes in the string, so it may not behave entirely as expected for non-ASCII strings.
///
/// For a detailed explanation, check the example on Wikipedia: <https://en.wikipedia.org/wiki/Levenshtein_distance>\
/// (see the examples with the matrices, for instance between KITTEN and SITTING)
///
/// Note that although we compute a matrix, left-to-right, top-to-bottom, at each step all we need to compute `cell[i][j]` is:
///   - `cell[i][j-1]`
///   - `cell[i-j][j]`
///   - `cell[i-i][j-1]`
///
/// This can be achieved by only using one "rolling" row and one additional variable, when computed `cell[i][j]` (or `row[i]`):
///   - `cell[i][j-1]` is the value to the left, on the same row (the one we just computed, `row[i-1]`)
///   - `cell[i-1][j]` is the value at `row[i]`, the one we're changing
///   - `cell[i-1][j-1]` was the value at `row[i-1]` before we changed it, for that we'll use a variable
///
/// Doing this reduces space complexity from O(nm) to O(n)
///
/// Second note: if we want to minimize space, since we're now O(n) make sure you use the shortest string horizontally, and the longest vertically
///
/// # Complexity
///   - time complexity: O(nm),
///   - space complexity: O(n),
///
/// where n and m are lengths of `str_a` and `str_b`
pub fn levenshtein_distance(string1: &str, string2: &str) -> usize {
    if string1.is_empty() {
        return string2.len();
    }
    let l1 = string1.len();
    let mut prev_dist: Vec<usize> = (0..=l1).collect();

    for (row, c2) in string2.chars().enumerate() {
        let mut prev_substitution_cost = prev_dist[0]; // we'll keep a reference to matrix[i-1][j-1] (top-left cell)
        prev_dist[0] = row + 1; // diff with empty string, since `row` starts at 0, it's `row + 1`

        for (col, c1) in string1.chars().enumerate() {
            let deletion_cost = prev_dist[col] + 1; // "on the left" in the matrix (i.e. the value we just computed)
            let insertion_cost = prev_dist[col + 1] + 1; // "on the top" in the matrix (means previous)
            let substitution_cost = if c1 == c2 {
                prev_substitution_cost // last char is the same on both ends, so the min_distance is left unchanged from matrix[i-1][i+1]
            } else {
                prev_substitution_cost + 1 // substitute the last character
            };

            prev_substitution_cost = prev_dist[col + 1]; // save the old value at (i-1, j-1)
            prev_dist[col + 1] = min3(deletion_cost, insertion_cost, substitution_cost);
        }
    }
    prev_dist[l1]
}

#[cfg(test)]
mod levenshtein_distance_should {
    use super::levenshtein_distance;

    #[test]
    fn test_doc_example() {
        assert_eq!(2, levenshtein_distance("FROG", "DOG"));
    }

    #[test]
    fn return_0_with_empty_strings() {
        assert_eq!(0, levenshtein_distance("", ""));
    }

    #[test]
    fn return_1_with_empty_and_a() {
        assert_eq!(1, levenshtein_distance("", "a"));
    }

    #[test]
    fn return_1_with_a_and_empty() {
        assert_eq!(1, levenshtein_distance("a", ""));
    }

    #[test]
    fn return_1_with_ab_and_a() {
        assert_eq!(1, levenshtein_distance("ab", "a"));
    }

    #[test]
    fn return_0_with_foobar_and_foobar() {
        assert_eq!(0, levenshtein_distance("foobar", "foobar"));
    }

    #[test]
    fn return_6_with_foobar_and_barfoo() {
        assert_eq!(6, levenshtein_distance("foobar", "barfoo"));
    }

    #[test]
    fn return_1_with_kind_and_bind() {
        assert_eq!(1, levenshtein_distance("kind", "bind"));
    }

    #[test]
    fn return_3_with_winner_and_win() {
        assert_eq!(3, levenshtein_distance("winner", "win"));
    }

    #[test]
    fn equal_strings() {
        assert_eq!(0, levenshtein_distance("Hello, world!", "Hello, world!"));
        assert_eq!(0, levenshtein_distance("Hello, world!", "Hello, world!"));
        assert_eq!(0, levenshtein_distance("Test_Case_#1", "Test_Case_#1"));
        assert_eq!(0, levenshtein_distance("Test_Case_#1", "Test_Case_#1"));
    }

    #[test]
    fn one_edit_difference() {
        assert_eq!(1, levenshtein_distance("Hello, world!", "Hell, world!"));
        assert_eq!(1, levenshtein_distance("Test_Case_#1", "Test_Case_#2"));
        assert_eq!(1, levenshtein_distance("Test_Case_#1", "Test_Case_#10"));
        assert_eq!(1, levenshtein_distance("Hello, world!", "Hell, world!"));
        assert_eq!(1, levenshtein_distance("Test_Case_#1", "Test_Case_#2"));
        assert_eq!(1, levenshtein_distance("Test_Case_#1", "Test_Case_#10"));
    }

    #[test]
    fn several_differences() {
        assert_eq!(2, levenshtein_distance("My Cat", "My Case"));
        assert_eq!(7, levenshtein_distance("Hello, world!", "Goodbye, world!"));
        assert_eq!(6, levenshtein_distance("Test_Case_#3", "Case #3"));
        assert_eq!(2, levenshtein_distance("My Cat", "My Case"));
        assert_eq!(7, levenshtein_distance("Hello, world!", "Goodbye, world!"));
        assert_eq!(6, levenshtein_distance("Test_Case_#3", "Case #3"));
    }
}

fn min3(a: usize, b: usize, c: usize) -> usize {
    min(a, min(b, c))
}

#[cfg(test)]
mod min3_should {
    use super::min3;

    #[test]
    fn return_1_with_1_2_3() {
        assert_eq!(1, min3(1, 2, 3));
    }

    #[test]
    fn return_1_with_3_2_1() {
        assert_eq!(1, min3(3, 2, 1));
    }

    #[test]
    fn return_1_with_2_3_1() {
        assert_eq!(1, min3(2, 3, 1));
    }
}
