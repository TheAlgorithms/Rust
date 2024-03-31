use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

/// Here's a basic (naive) implementation for generating permutations
pub fn permute<T: Clone + Debug>(arr: &[T]) -> Vec<Vec<T>> {
    if arr.is_empty() {
        return vec![vec![]];
    }
    let n = arr.len();
    let count = (1..=n).product(); // n! permutations
    let mut collector = Vec::with_capacity(count); // collects the permuted arrays
    let mut arr = arr.to_owned(); // we'll need to mutate the array

    // the idea is the following: imagine [a, b, c]
    // always swap an item with the last item, then generate all permutations from the first k characters
    // permute_recurse(arr, k - 1, collector); // leave the last character alone, and permute the first k-1 characters
    permute_recurse(&mut arr, n, &mut collector);
    collector
}

fn permute_recurse<T: Clone + Debug>(arr: &mut Vec<T>, k: usize, collector: &mut Vec<Vec<T>>) {
    if k == 1 {
        collector.push(arr.to_owned());
        return;
    }
    for i in 0..k {
        arr.swap(i, k - 1); // swap i with the last character
        permute_recurse(arr, k - 1, collector); // collect the permutations of the rest
        arr.swap(i, k - 1); // swap back to original
    }
}

/// A common variation of generating permutations is to generate only unique permutations
/// Of course, we could use the version above together with a Set as collector instead of a Vec.
/// But let's try something different: how can we avoid to generate duplicated permutations in the first place, can we tweak the algorithm above?
pub fn permute_unique<T: Clone + Debug + Eq + Hash + Copy>(arr: &[T]) -> Vec<Vec<T>> {
    if arr.is_empty() {
        return vec![vec![]];
    }
    let n = arr.len();
    let count = (1..=n).product(); // n! permutations
    let mut collector = Vec::with_capacity(count); // collects the permuted arrays
    let mut arr = arr.to_owned(); // Heap's algorithm needs to mutate the array
    permute_recurse_unique(&mut arr, n, &mut collector);
    collector
}

fn permute_recurse_unique<T: Clone + Debug + Eq + Hash + Copy>(
    arr: &mut Vec<T>,
    k: usize,
    collector: &mut Vec<Vec<T>>,
) {
    // We have the same base-case as previously, whenever we reach the first element in the array, collect the result
    if k == 1 {
        collector.push(arr.to_owned());
        return;
    }
    // We'll keep the same idea (swap with last item, and generate all permutations for the first k - 1)
    // But we'll have to be careful though: how would we generate duplicates?
    // Basically if, when swapping i with k-1, we generate the exact same array as in a previous iteration
    // Imagine [a, a, b]
    // i = 0:
    //  Swap (a, b) => [b, a, a], fix 'a' as last, and generate all permutations of [b, a] => [b, a, a], [a, b, a]
    //  Swap Back to [a, a, b]
    // i = 1:
    //  Swap(a, b) => [b, a, a], we've done that already!!
    let mut swapped = HashSet::with_capacity(k);
    for i in 0..k {
        if swapped.contains(&arr[i]) {
            continue;
        }
        swapped.insert(arr[i]);
        arr.swap(i, k - 1); // swap i with the last character
        permute_recurse_unique(arr, k - 1, collector); // collect the permutations
        arr.swap(i, k - 1); // go back to original
    }
}

#[cfg(test)]
mod tests {
    use crate::general::permutations::naive::{permute, permute_unique};
    use crate::general::permutations::tests::{
        assert_permutations, assert_valid_permutation, NotTooBigVec,
    };
    use quickcheck_macros::quickcheck;
    use std::collections::HashSet;

    #[test]
    fn test_3_different_values() {
        let original = vec![1, 2, 3];
        let res = permute(&original);
        assert_eq!(res.len(), 6); // 3!
        for permut in res {
            assert_valid_permutation(&original, &permut)
        }
    }

    #[test]
    fn empty_array() {
        let empty: std::vec::Vec<u8> = vec![];
        assert_eq!(permute(&empty), vec![vec![]]);
        assert_eq!(permute_unique(&empty), vec![vec![]]);
    }

    #[test]
    fn test_3_times_the_same_value() {
        let original = vec![1, 1, 1];
        let res = permute(&original);
        assert_eq!(res.len(), 6); // 3!
        for permut in res {
            assert_valid_permutation(&original, &permut)
        }
    }

    #[quickcheck]
    fn test_some_elements(NotTooBigVec { inner: original }: NotTooBigVec) {
        let permutations = permute(&original);
        assert_permutations(&original, &permutations)
    }

    #[test]
    fn test_unique_values() {
        let original = vec![1, 1, 2, 2];
        let unique_permutations = permute_unique(&original);
        let every_permutation = permute(&original);
        for unique_permutation in &unique_permutations {
            assert!(every_permutation.contains(unique_permutation));
        }
        assert_eq!(
            unique_permutations.len(),
            every_permutation.iter().collect::<HashSet<_>>().len()
        )
    }
}
