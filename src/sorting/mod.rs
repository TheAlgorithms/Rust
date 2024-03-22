/* start auto-imports */
mod bead_sort;
mod binary_insertion_sort;
mod bingo_sort;
mod bitonic_sort;
mod bogo_sort;
mod bubble_sort;
mod bucket_sort;
mod cocktail_shaker_sort;
mod comb_sort;
mod counting_sort;
mod cycle_sort;
mod dutch_national_flag_sort;
mod exchange_sort;
mod gnome_sort;
mod heap_sort;
mod insertion_sort;
mod intro_sort;
mod merge_sort;
mod odd_even_sort;
mod pancake_sort;
mod patience_sort;
mod pigeonhole_sort;
mod quick_sort;
mod quick_sort_3_ways;
mod radix_sort;
mod selection_sort;
mod shell_sort;
mod sleep_sort;
mod sort_utils;
mod stooge_sort;
mod tim_sort;
mod tree_sort;
mod wave_sort;
mod wiggle_sort;
pub use bead_sort::*;
pub use binary_insertion_sort::*;
pub use bingo_sort::*;
pub use bitonic_sort::*;
pub use bogo_sort::*;
pub use bubble_sort::*;
pub use bucket_sort::*;
pub use cocktail_shaker_sort::*;
pub use comb_sort::*;
pub use counting_sort::*;
pub use cycle_sort::*;
pub use dutch_national_flag_sort::*;
pub use exchange_sort::*;
pub use gnome_sort::*;
pub use heap_sort::*;
pub use insertion_sort::*;
pub use intro_sort::*;
pub use merge_sort::*;
pub use odd_even_sort::*;
pub use pancake_sort::*;
pub use patience_sort::*;
pub use pigeonhole_sort::*;
pub use quick_sort::*;
pub use quick_sort_3_ways::*;
pub use radix_sort::*;
pub use selection_sort::*;
pub use shell_sort::*;
pub use sleep_sort::*;
pub use sort_utils::*;
pub use stooge_sort::*;
pub use tim_sort::*;
pub use tree_sort::*;
pub use wave_sort::*;
pub use wiggle_sort::*;
/* end auto-imports */

#[cfg(test)]
use std::cmp;

#[cfg(test)]
pub fn have_same_elements<T>(a: &[T], b: &[T]) -> bool
where
    // T: cmp::PartialOrd,
    // If HashSet is used
    T: cmp::PartialOrd + cmp::Eq + std::hash::Hash,
{
    use std::collections::HashSet;

    match a.len() == b.len() {
        true => {
            // This is O(n^2) but performs better on smaller data sizes
            //b.iter().all(|item| a.contains(item))

            // This is O(n), performs well on larger data sizes
            let set_a: HashSet<&T> = a.iter().collect();
            let set_b: HashSet<&T> = b.iter().collect();
            set_a == set_b
        }
        false => false,
    }
}

#[cfg(test)]
pub fn is_sorted<T>(arr: &[T]) -> bool
where
    T: cmp::PartialOrd,
{
    arr.windows(2).all(|w| w[0] <= w[1])
}

#[cfg(test)]
pub fn is_descending_sorted<T>(arr: &[T]) -> bool
where
    T: cmp::PartialOrd,
{
    arr.windows(2).all(|w| w[0] >= w[1])
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_sorted() {
        use super::*;

        assert!(is_sorted(&[] as &[isize]));
        assert!(is_sorted(&["a"]));
        assert!(is_sorted(&[1, 2, 3]));
        assert!(is_sorted(&[0, 1, 1]));

        assert!(!is_sorted(&[1, 0]));
        assert!(!is_sorted(&[2, 3, 1, -1, 5]));
    }
}