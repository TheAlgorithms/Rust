/// Finds the longest increasing subsequence and returns it.
///
/// If multiple subsequences with the longest possible subsequence length can be found, the
/// subsequence which appeared first will be returned (see `test_example_1`).
///
/// Inspired by [this LeetCode problem](https://leetcode.com/problems/longest-increasing-subsequence/).
pub fn longest_increasing_subsequence<T: Ord + Clone>(input_array: &[T]) -> Vec<T> {
    let n = input_array.len();
    if n <= 1 {
        return input_array.to_vec();
    }

    let mut increasing_sequence: Vec<(T, usize)> = Vec::new();
    let mut previous = vec![0_usize; n];

    increasing_sequence.push((input_array[0].clone(), 1));
    for i in 1..n {
        let value = input_array[i].clone();
        if value > increasing_sequence.last().unwrap().0 {
            previous[i] = increasing_sequence.last().unwrap().1 - 1;
            increasing_sequence.push((value, i + 1));
            continue;
        }

        let change_position = increasing_sequence
            .binary_search(&(value.clone(), 0))
            .unwrap_or_else(|x| x);
        increasing_sequence[change_position] = (value, i + 1);
        previous[i] = match change_position {
            0 => i,
            other => increasing_sequence[other - 1].1 - 1,
        };
    }

    // Construct subsequence
    let mut out: Vec<T> = Vec::with_capacity(increasing_sequence.len());

    out.push(increasing_sequence.last().unwrap().0.clone());
    let mut current_index = increasing_sequence.last().unwrap().1 - 1;
    while previous[current_index] != current_index {
        current_index = previous[current_index];
        out.push(input_array[current_index].clone());
    }

    out.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::longest_increasing_subsequence;

    #[test]
    /// Need to specify generic type T in order to function
    fn test_empty_vec() {
        assert_eq!(longest_increasing_subsequence::<i32>(&[]), vec![]);
    }

    #[test]
    fn test_example_1() {
        assert_eq!(
            longest_increasing_subsequence(&[10, 9, 2, 5, 3, 7, 101, 18]),
            vec![2, 3, 7, 18]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            longest_increasing_subsequence(&[0, 1, 0, 3, 2, 3]),
            vec![0, 1, 2, 3]
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            longest_increasing_subsequence(&[7, 7, 7, 7, 7, 7, 7]),
            vec![7]
        );
    }

    #[test]
    fn test_tle() {
        let mut input_array = vec![0i64; 1e5 as usize];
        let mut expected_result: Vec<i64> = Vec::with_capacity(5e4 as usize);
        for (idx, num) in input_array.iter_mut().enumerate() {
            match idx % 2 {
                0 => {
                    *num = idx as i64;
                    expected_result.push(*num);
                }
                1 => *num = -(idx as i64),
                _ => unreachable!(),
            }
        }
        expected_result[0] = -1;
        assert_eq!(
            longest_increasing_subsequence(&input_array),
            expected_result
        );
        // should be [-1, 2, 4, 6, 8, ...]
        // the first number is not 0, it would be replaced by -1 before 2 is added
    }

    #[test]
    fn test_negative_elements() {
        assert_eq!(longest_increasing_subsequence(&[-2, -1]), vec![-2, -1]);
    }
}
