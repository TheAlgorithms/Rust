use crate::math::PCG32;
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT: u64 = 4294967296;

fn is_sorted<T: Ord>(arr: &[T], len: usize) -> bool {
    // `windows` yields nothing below two elements, where `0..len - 1` underflowed.
    arr[..len].windows(2).all(|pair| pair[0] <= pair[1])
}

#[cfg(target_pointer_width = "64")]
fn generate_index(range: usize, generator: &mut PCG32) -> usize {
    generator.get_u64() as usize % range
}

#[cfg(not(target_pointer_width = "64"))]
fn generate_index(range: usize, generator: &mut PCG32) -> usize {
    generator.get_u32() as usize % range
}

/**
 * Fisher–Yates shuffle for generating random permutation.
 */
fn permute_randomly<T>(arr: &mut [T], len: usize, generator: &mut PCG32) {
    for i in (1..len).rev() {
        let j = generate_index(i + 1, generator);
        arr.swap(i, j);
    }
}

pub fn bogo_sort<T: Ord>(arr: &mut [T]) {
    let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_millis() as u64,
        Err(_) => DEFAULT,
    };

    let mut random_generator = PCG32::new_default(seed);

    let arr_length = arr.len();
    while !is_sorted(arr, arr_length) {
        permute_randomly(arr, arr_length, &mut random_generator);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_array() {
        let mut arr = [1, 8, 3, 2, 7, 4, 6, 5];
        bogo_sort(&mut arr);

        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    #[test]
    fn sorted_array() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7, 8];
        bogo_sort(&mut arr);

        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    #[test]
    fn empty() {
        let mut arr: Vec<i32> = vec![];
        bogo_sort(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn one_element() {
        let mut arr = [7];
        bogo_sort(&mut arr);
        assert_eq!(&arr, &[7]);
    }
}
