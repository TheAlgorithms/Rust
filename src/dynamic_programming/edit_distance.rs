// Edit distance problem. See https://leetcode.com/problems/edit-distance/description/.
//
// We solve this problem with dynamic programming and tabulation. For string W1 of length N and
// string W2 of length M, we create a table of size (N+1, M+1). For i,j > 0, table[i][j] holds the
// edit distance between strings W1[0..i-1] and W2[0..j-1]. The zero'th row/index holds matching
// the empty string. It is initialized to 0, 1, 2, .., (N|M) because matching against the empty
// string consists only of (N|M) insert/delete edits.
//
// For i,j > 0, we compute table[i][j] with the following rules:
// 1. If W1[i-1] == W2[j-1], then table[i][j] = table[i-1][j-1]. We do not need any edits because
//    the characters match.
// 2. If W1[i-1] != W2[j-1], add one edit to the minimum of the following:
//   a. table[i-1][j]. Insert one character to the end of W1.
//   b. table[i][j-1]. Insert one character to the end of W2.
//   c. table[i-1][j-1]. Replace the last character so that they match.
//   We set table[i][j] = 1 + min(a, b, c)

pub fn edit_distance(word1: String, word2: String) -> i32 {
    let n = word1.len();
    let m = word2.len();

    let mut table: Vec<Vec<i32>> = vec![vec![0; m + 1]; n + 1];
    for (i, row) in table.iter_mut().enumerate() {
        row[0] = i as i32;
    }
    for i in 1..m + 1 {
        table[0][i] = i as i32;
    }

    for (i, c1) in word1.chars().enumerate() {
        for (j, c2) in word2.chars().enumerate() {
            if c1 == c2 {
                table[i + 1][j + 1] = table[i][j];
            } else {
                let mut matched = std::cmp::min(table[i][j + 1], table[i + 1][j]);
                matched = std::cmp::min(matched, table[i][j]);
                table[i + 1][j + 1] = matched + 1;
            }
        }
    }

    table[n][m]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(edit_distance("a".to_string(), "b".to_string()), 1);
        assert_eq!(edit_distance("horse".to_string(), "ros".to_string()), 3);
        assert_eq!(
            edit_distance("intention".to_string(), "execution".to_string()),
            5
        );
        assert_eq!(
            edit_distance("rust".to_string(), "thealgorithms".to_string()),
            12
        );
    }

    #[test]
    fn empty() {
        assert_eq!(edit_distance("".to_string(), "".to_string()), 0);
        assert_eq!(edit_distance("".to_string(), "foobar".to_string()), 6);
        assert_eq!(
            edit_distance("thealgorithms".to_string(), "".to_string()),
            13
        );
    }

    #[test]
    fn identical() {
        assert_eq!(edit_distance("foobar".to_string(), "foobar".to_string()), 0);
        assert_eq!(
            edit_distance("thealgorithms".to_string(), "thealgorithms".to_string()),
            0
        );
    }
}
