fn match_with_z_array<T: Eq>(
    input_string: &[T],
    pattern: &[T],
    start_index: usize,
    only_full_matches: bool,
) -> Vec<usize> {
    let size = input_string.len();
    let pattern_size = pattern.len();
    let mut last_match: usize = 0;
    let mut match_end: usize = 0;
    let mut array = vec![0usize; size];
    for i in start_index..size {
        // getting plain z array of a string requires matching from index
        // 1 instead of 0 (which gives a trivial result instead)
        if i <= match_end {
            array[i] = std::cmp::min(array[i - last_match], match_end - i + 1);
        }
        while (i + array[i]) < size && array[i] < pattern_size {
            if input_string[i + array[i]] != pattern[array[i]] {
                break;
            }
            array[i] += 1;
        }
        if (i + array[i]) > (match_end + 1) {
            match_end = i + array[i] - 1;
            last_match = i;
        }
    }
    if !only_full_matches {
        array
    } else {
        let mut answer: Vec<usize> = vec![];
        for (idx, number) in array.iter().enumerate() {
            if *number == pattern_size {
                answer.push(idx);
            }
        }
        answer
    }
}

pub fn z_array<T: Eq>(input: &[T]) -> Vec<usize> {
    match_with_z_array(input, input, 1, false)
}

pub fn match_pattern<T: Eq>(input: &[T], pattern: &[T]) -> Vec<usize> {
    match_with_z_array(input, pattern, 0, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_array() {
        let string = "aabaabab";
        let array = z_array(string.as_bytes());
        assert_eq!(array, vec![0, 1, 0, 4, 1, 0, 1, 0]);
    }

    #[test]
    fn pattern_in_text() {
        let text: &str = concat!(
            "lorem ipsum dolor sit amet, consectetur ",
            "adipiscing elit, sed do eiusmod tempor ",
            "incididunt ut labore et dolore magna aliqua"
        );
        let pattern1 = "rem";
        let pattern2 = "em";
        let pattern3 = ";alksdjfoiwer";
        let pattern4 = "m";

        assert_eq!(match_pattern(text.as_bytes(), pattern1.as_bytes()), vec![2]);
        assert_eq!(
            match_pattern(text.as_bytes(), pattern2.as_bytes()),
            vec![3, 73]
        );
        assert_eq!(match_pattern(text.as_bytes(), pattern3.as_bytes()), vec![]);
        assert_eq!(
            match_pattern(text.as_bytes(), pattern4.as_bytes()),
            vec![4, 10, 23, 68, 74, 110]
        );

        let text2 = "aaaaaaaa";
        let pattern5 = "aaa";
        assert_eq!(
            match_pattern(text2.as_bytes(), pattern5.as_bytes()),
            vec![0, 1, 2, 3, 4, 5]
        )
    }

    #[test]
    fn long_pattern_in_text() {
        let text = vec![65u8; 1e5 as usize];
        let pattern = vec![65u8; 5e4 as usize];

        let mut expected_answer = vec![0usize; (1e5 - 5e4 + 1f64) as usize];
        for (idx, i) in expected_answer.iter_mut().enumerate() {
            *i = idx;
        }
        assert_eq!(
            match_pattern(text.as_slice(), pattern.as_slice()),
            expected_answer
        );
    }
}
