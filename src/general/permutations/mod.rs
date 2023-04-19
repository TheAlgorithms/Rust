mod heap;

pub use self::heap::heap_permute;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    pub(crate) fn assert_valid_permutation(original: &[i32], permuted: &[i32]) {
        assert_eq!(original.len(), permuted.len());
        let mut indices = HashMap::with_capacity(original.len());
        for value in original {
            *indices.entry(*value).or_insert(0) += 1;
        }
        for permut_value in permuted {
            let count = indices.get_mut(permut_value).unwrap_or_else(|| {
                panic!(
                    "Value {} appears too many times in permutation",
                    permut_value,
                )
            });
            *count -= 1; // use this value
            if *count == 0 {
                indices.remove(permut_value); // so that we can simply check every value has been removed properly
            }
        }
        assert!(indices.is_empty())
    }

    #[test]
    fn test_valid_permutations() {
        assert_valid_permutation(&[1, 2, 3], &[1, 2, 3]);
        assert_valid_permutation(&[1, 2, 3], &[1, 3, 2]);

        assert_valid_permutation(&[1, 2, 3], &[2, 1, 3]);
        assert_valid_permutation(&[1, 2, 3], &[2, 3, 1]);

        assert_valid_permutation(&[1, 2, 3], &[3, 1, 2]);
        assert_valid_permutation(&[1, 2, 3], &[3, 2, 1]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_permutation_1() {
        assert_valid_permutation(&[1, 2, 3], &[4, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_permutation_2() {
        assert_valid_permutation(&[1, 2, 3], &[1, 4, 3]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_permutation_3() {
        assert_valid_permutation(&[1, 2, 3], &[1, 2, 4]);
    }

    #[test]
    #[should_panic]
    fn test_invalid_permutation_repeat() {
        assert_valid_permutation(&[1, 2, 3], &[1, 2, 2]);
    }
}
