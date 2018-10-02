// The longest common subsequence (LCS) problem is
// the problem of finding the longest subsequence common to all sequences
// in a set of sequences.

// lcs is case sensitive, and treats different cases as unique.

pub fn lcs(str1: &str, str2: &str) -> Vec<char> {
    let mut n = str1.len();
    let mut m = str2.len();
    let chr1: Vec<char> = str1.chars().collect();
    let chr2: Vec<char> = str2.chars().collect();

    let mut matrix = vec![vec![0i8; m + 1]; n + 1];
    let mut dir = vec![vec![0i8; m + 1]; n + 1];
    for i in 0..=n {
        for j in 0..=m {
            if i == 0 || j == 0 {
                dir[i][j] = -1;
                matrix[i][j] = 0;
            } else if chr1[i - 1] == chr2[j - 1] {
                dir[i][j] = 0;
                matrix[i][j] = matrix[i - 1][j - 1] + 1;
            } else if matrix[i - 1][j] >= matrix[i][j - 1] {
                dir[i][j] = 1;
                matrix[i][j] = matrix[i - 1][j];
            } else {
                dir[i][j] = 2;
                matrix[i][j] = matrix[i][j - 1];
            }
        }
    }

    let mut lcs: Vec<char> = Vec::new();
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
    use dynamic_program::lcs;

    #[test]
    fn simple_lcs() {
        let x = "ABBADE";
        let y = "ABCDEF";
        assert_eq!(lcs(x, y), ['A', 'B', 'D', 'E']);
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
