mod heap;
mod naive;
mod steinhaus_johnson_trotter;

pub use self::heap::heap_permute;
pub use self::naive::{permute, permute_unique};
pub use self::steinhaus_johnson_trotter::steinhaus_johnson_trotter_permute;

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};
    use std::collections::HashMap;

    pub fn assert_permutations(original: &[i32], permutations: &[Vec<i32>]) {
        if original.is_empty() {
            assert_eq!(vec![vec![] as Vec<i32>], permutations);
            return;
        }
        let n = original.len();
        assert_eq!((1..=n).product::<usize>(), permutations.len()); // n!
        for permut in permutations {
            assert_valid_permutation(original, permut);
        }
    }

    pub fn assert_valid_permutation(original: &[i32], permuted: &[i32]) {
        assert_eq!(original.len(), permuted.len());
        let mut indices = HashMap::with_capacity(original.len());
        for value in original {
            *indices.entry(*value).or_insert(0) += 1;
        }
        for permut_value in permuted {
            let count = indices.get_mut(permut_value).unwrap_or_else(|| {
                panic!("Value {permut_value} appears too many times in permutation")
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

    /// A Data Structure for testing permutations
    /// Holds a Vec<i32> with just a few items, so that it's not too long to compute permutations
    #[derive(Debug, Clone)]
    pub struct NotTooBigVec {
        pub(crate) inner: Vec<i32>, // opaque type alias so that we can implement Arbitrary
    }

    const MAX_SIZE: usize = 8; // 8! ~= 40k permutations already
    impl Arbitrary for NotTooBigVec {
        fn arbitrary(g: &mut Gen) -> Self {
            let size = usize::arbitrary(g) % MAX_SIZE;
            let res = (0..size).map(|_| i32::arbitrary(g)).collect();
            NotTooBigVec { inner: res }
        }
    }
}
