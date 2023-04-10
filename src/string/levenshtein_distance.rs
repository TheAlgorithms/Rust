use std::cmp::min;

// Let's imagine the distance between FROG and DOG (string1 = "FROG", string2 = "DOG")
// The distance would be 2: Keep 'OG', remove F, and substitute 'R' for a 'D'
//
// In order to compute the distance, we are going to build the following matrix:
//     |  ∅  |  F  |  R  |  O  |  G  |
//  ∅  |  0  |  1  |  2  |  3  |  4  |
//  D  |  1  |     |     |     |     |
//  O  |  2  |     |     |     |     |
//  G  |  3  |     |     |     |     |
// Where:
//  * ∅ indicates an empty String
//  * each cell (i, j) in the matrix indicates the minimum distance between substrings string1[0..=j] and string2[0..=i]
//      * (but considering we start with empty strings)
// Above, we have filled the defaults:
//   -> The distance between any string and an empty string is the string length
// For example: the distance between String "F" and an empty String is 1, dist("FR", ∅) = 2, etc.
// In the same fashion, vertically: dist("DOG", ∅) is 3
// What we are interested in, is the last value at the bottom right of the matrix, which indicates dist("FROG", "DOG")
//
// How do we compute each cell in the matrix?
//  -> Let's do this top-to-bottom, left-to-right
// At each step we can either:
//      * insert a char
//      * delete a char
//      * substitute a char
// Say we're evaluating cell at index `(3, 2)` <=> `dist("FR", "DOG")`
//  * "Inserting one char" means incrementing by one `dist("F", "DOG")` (cell at index: `(3, 1)`)
//  * "Deleting one char" means incrementing by one `dist("FR", "DO")` (cell at index: `(2, 2)`)
//  * "Substituting one char" means incrementing by one `dist("F", "DO")` (cell at index: `(2, 1)` if and only if chars differ (here 'R' != 'G'), otherwise we can just keep that distance
// This would give the following:
//     |  ∅  |  F  |  R  |  O  |  G  |
//  ∅  |  0  |  1  |  2  |  3  |  4  |
//  D  |  1  |  1  |  2  |  3  |  4  |
//  O  |  2  |  2  |  2  |  2  |  3  |
//  G  |  3  |  3  |  3  |  3  |  2  |
//
// This means when we evaluate a cell we only need the current row (the previous value), and the previous row.
// We therefore don't need to keep the full matrix in memory, but only previous row
pub fn levenshtein_distance(string1: &str, string2: &str) -> usize {
    if string1.is_empty() {
        return string2.len();
    }
    let l1 = string1.len();

    // Let's start by building the first row, in the example:
    //     |  ∅  |  F  |  R  |  O  |  G  |
    //  ∅  |  0  |  1  |  2  |  3  |  4  |
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
