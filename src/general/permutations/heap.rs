use std::fmt::Debug;

/// Computes all permutations of an array using Heap's algorithm
pub fn heap_permute<T: Clone + Debug>(arr: &[T]) -> Vec<Vec<T>> {
    if arr.is_empty() {
        return vec![vec![]];
    }
    let n = arr.len();
    let mut collector = Vec::with_capacity((1..=n).product());
    let mut arr = arr.to_owned();
    heap_recurse(&mut arr, n, &mut collector);
    collector
}

fn heap_recurse<T: Clone + Debug>(arr: &mut [T], k: usize, collector: &mut Vec<Vec<T>>) {
    if k == 1 {
        // base-case
        // collect the array as it is now
        collector.push((*arr).to_owned());
        return;
    }
    // imagine we have [a, b, c]
    heap_recurse(arr, k - 1, collector); // leave 'c' alone, and compute all permutations of [a, b]

    // after this point we're done with permutations([a, b]) + c
    for i in 0..(k - 1) {
        // now deal with [a, b]
        if k % 2 == 0 {
            arr.swap(i, k - 1);
        } else {
            arr.swap(0, k - 1);
        }
        heap_recurse(arr, k - 1, collector);
    }
}

#[cfg(test)]
mod tests {
    // use quickcheck_macros::quickcheck;

    use crate::general::permutations::heap_permute;
    use crate::general::permutations::tests::assert_valid_permutation;

    // #[quickcheck]
    // fn test_case_1(original: Vec<i32>) {}

    #[test]
    fn test_3_different_values() {
        let original = vec![1, 2, 3];
        let res = heap_permute(&original);
        assert_eq!(res.len(), 6); // 3!
        for permut in res {
            assert_valid_permutation(&original, &permut)
        }
    }

    #[test]
    fn test_3_times_the_same_value() {
        let original = vec![1, 1, 1];
        let res = heap_permute(&original);
        assert_eq!(res.len(), 6); // 3!
        for permut in res {
            assert_valid_permutation(&original, &permut)
        }
    }
}
