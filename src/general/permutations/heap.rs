use std::fmt::Debug;

/// Computes all permutations of an array using Heap's algorithm
/// Read `recurse_naive` first, since we're building on top of the same intuition
pub fn heap_permute<T: Clone + Debug>(arr: &[T]) -> Vec<Vec<T>> {
    if arr.is_empty() {
        return vec![vec![]];
    }
    let n = arr.len();
    let mut collector = Vec::with_capacity((1..=n).product()); // collects the permuted arrays
    let mut arr = arr.to_owned(); // Heap's algorithm needs to mutate the array
    heap_recurse(&mut arr, n, &mut collector);
    collector
}

fn heap_recurse<T: Clone + Debug>(arr: &mut [T], k: usize, collector: &mut Vec<Vec<T>>) {
    if k == 1 {
        // same base-case as in the naive version
        collector.push((*arr).to_owned());
        return;
    }
    // Remember the naive recursion. We did the following: swap(i, last), recurse, swap back(i, last)
    // Heap's algorithm has a more clever way of permuting the elements so that we never need to swap back!
    for i in 0..k {
        // now deal with [a, b]
        let swap_idx = if k % 2 == 0 { i } else { 0 };
        arr.swap(swap_idx, k - 1);
        heap_recurse(arr, k - 1, collector);
    }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::general::permutations::heap_permute;
    use crate::general::permutations::tests::{
        assert_permutations, assert_valid_permutation, NotTooBigVec,
    };

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

    #[quickcheck]
    fn test_some_elements(NotTooBigVec { inner: original }: NotTooBigVec) {
        let permutations = heap_permute(&original);
        assert_permutations(&original, &permutations)
    }
}
