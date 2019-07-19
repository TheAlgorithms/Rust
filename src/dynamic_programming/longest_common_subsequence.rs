/// # Longest Common Subsequence

/// `longest_common_subsequence(v1, v2)` Given two sequences find the longest common subsequence.
///     If there are multiple common subsequences with the same maximum length, return any one of them.
///
/// Assumptions: 1 <= v1 <= 100
///              1 <= v2 <= 100
pub fn longest_common_subsequence(v1: Vec<&str>, v2: Vec<&str>) -> String {
    let mut memo = vec![vec![0; v1.len() + 1]; v2.len() + 1];

    for i in 1..=v2.len() {
        for j in 1..=v1.len() {
            let diagonal = memo[i - 1][j - 1];
            let up = memo[i - 1][j];
            let left = memo[i][j - 1];

            if v2[i - 1] == v1[j - 1] {
                memo[i][j] = 1 + diagonal;
            } else {
                memo[i][j] = usize::max(left, up);
            }
        }
    }

    let mut solution: Vec<&str> = vec![];
    let mut i = v2.len();
    let mut j = v1.len();

    while i > 0 && j > 0 {
        let _i = i - 1;
        let _j = j - 1;

        let curr_count = memo[i][j];
        let curr_v2_letter = v2[_i];
        let curr_v1_letter = v1[_j];

        if curr_v1_letter == curr_v2_letter {
            i = _i;
            j = _j;
            solution.push(curr_v2_letter);
        } else {
            if curr_count == memo[i][_j] {
                j = _j;
            } else {
                i = _i;
            }
        }
    }

    solution.reverse();
    solution.join(" ")
}

#[cfg(test)]
mod tests {
    use super::longest_common_subsequence;

    fn format_for_testing(s: &str) -> Vec<&str> {
        s.split_whitespace().collect()
    }

    #[test]
    fn test_one() {
        assert_eq!(
            longest_common_subsequence(
                format_for_testing("1 2 3 4 1"),
                format_for_testing("3 4 1 2 1 3")
            ),
            "1 2 3".to_owned().to_string()
        );
    }

    #[test]
    fn test_two() {
        assert_eq!(
            longest_common_subsequence(
                format_for_testing("3 9 8 3 9 7 9 7 0"),
                format_for_testing("3 3 9 9 9 1 7 2 0 6")
            ),
            "3 9 9 9 7 0".to_owned().to_string()
        );
    }

    #[test]
    fn test_three() {
        assert_eq!(
            longest_common_subsequence(
                format_for_testing("16 27 89 79 60 76 24 88 55 94 57 42 56 74 24 95 55 33 69 29 14 7 94 41 8 71 12 15 43 3 23 49 84 78 73 63 5 46 98 26 40 76 41 89 24 20 68 14 88 26"), 
                format_for_testing("27 76 88 0 55 99 94 70 34 42 31 47 56 74 69 46 93 88 89 7 94 41 68 37 8 71 57 15 43 89 43 3 23 35 49 38 84 98 47 89 73 24 20 14 88 75")
            ),
            "27 76 88 55 94 42 56 74 69 7 94 41 8 71 15 43 3 23 49 84 98 89 24 20 14 88".to_owned().to_string()
        );
    }

}
