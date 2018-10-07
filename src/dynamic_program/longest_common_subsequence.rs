/// The longest common subsequence (LCS) problem is
/// the problem of finding the longest subsequence common to all sequences
/// in a set of sequences.
/// https://en.wikipedia.org/wiki/Longest_common_subsequence_problem

// lcs is case sensitive, and treats different cases as unique.

pub fn longest_common_subsequence(str1: &str, str2: &str) -> Vec<char> {
    let mut n = str1.len();
    let mut m = str2.len();
    let chr1: Vec<char> = str1.chars().collect();
    let chr2: Vec<char> = str2.chars().collect();

    // Build a matrix of size 'n'x'm'
    let mut matrix = vec![vec![0i8; m + 1]; n + 1];
    // Build a direction matrix of size 'n' x 'm'
    let mut dir = vec![vec![0i8; m + 1]; n + 1];

    // Fill the matrix
    for i in 0..=n {
        for j in 0..=m {
            // The 0th row of matrix is 0
            // The 0th row of direction matrix is -1
            if i == 0 || j == 0 {
                dir[i][j] = -1;
                matrix[i][j] = 0;
            }
            // It the letters match
            // Direction is 0 (Diagonal Top Left) and matrix[x][y] = value of (diagonal) top left cell + 1
            else if chr1[i - 1] == chr2[j - 1] {
                dir[i][j] = 0;
                matrix[i][j] = matrix[i - 1][j - 1] + 1;
            }
            // If value1 > value2
            // direction is 1 (Left) and matrix[x][y] = value of left cell
            else if matrix[i - 1][j] >= matrix[i][j - 1] {
                dir[i][j] = 1;
                matrix[i][j] = matrix[i - 1][j];
            }
            // If value2 > value1
            // Direction is 2 (Right) and matrix[x][y] = value of upper cell
            else {
                dir[i][j] = 2;
                matrix[i][j] = matrix[i][j - 1];
            }
        }
    }

    let mut lcs: Vec<char> = Vec::new();
    // Traverse the matrix from the bottom right corner
    // Using the directional matrix as guide
    while n > 0 && m > 0 {
        match dir[n][m] {
            0 => {
                lcs.push(chr1[n - 1]);
                n -= 1;
                m -= 1;
            }
            1 => n -= 1,
            2 => m -= 1,
            _ => (),
        };
    }
    lcs.into_iter().rev().collect()
}

#[cfg(test)]
mod test {
    use dynamic_program::longest_common_subsequence as lcs;

    #[test]
    fn simple_lcs() {
        assert_eq!(lcs("ABBADE", "ABCDEF"), ['A', 'B', 'D', 'E']);
    }

    #[test]
    fn uneven_string_lcs() {
        assert_eq!(lcs("ABDE", "ABCDF"), ['A', 'B', 'D']);
        assert_eq!(lcs("", "ABC"), []);
    }

    #[test]
    fn different_case_lcs() {
        assert_eq!(lcs("ABcd", "AbCd"), ['A', 'd']);
    }
}
