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
#[cfg(test)]
mod sort_utils;
mod stooge_sort;
mod tim_sort;
mod tree_sort;
mod wave_sort;
mod wiggle_sort;

pub use self::bead_sort::bead_sort;
pub use self::binary_insertion_sort::binary_insertion_sort;
pub use self::bingo_sort::bingo_sort;
pub use self::bitonic_sort::bitonic_sort;
pub use self::bogo_sort::bogo_sort;
pub use self::bubble_sort::bubble_sort;
pub use self::bucket_sort::bucket_sort;
pub use self::cocktail_shaker_sort::cocktail_shaker_sort;
pub use self::comb_sort::comb_sort;
pub use self::counting_sort::counting_sort;
pub use self::counting_sort::generic_counting_sort;
pub use self::cycle_sort::cycle_sort;
pub use self::dutch_national_flag_sort::dutch_national_flag_sort;
pub use self::exchange_sort::exchange_sort;
pub use self::gnome_sort::gnome_sort;
pub use self::heap_sort::heap_sort;
pub use self::insertion_sort::insertion_sort;
pub use self::intro_sort::intro_sort;
pub use self::merge_sort::bottom_up_merge_sort;
pub use self::merge_sort::top_down_merge_sort;
pub use self::odd_even_sort::odd_even_sort;
pub use self::pancake_sort::pancake_sort;
pub use self::patience_sort::patience_sort;
pub use self::pigeonhole_sort::pigeonhole_sort;
pub use self::quick_sort::{partition, quick_sort};
pub use self::quick_sort_3_ways::quick_sort_3_ways;
pub use self::radix_sort::radix_sort;
pub use self::selection_sort::selection_sort;
pub use self::shell_sort::shell_sort;
pub use self::sleep_sort::sleep_sort;
pub use self::stooge_sort::stooge_sort;
pub use self::tim_sort::tim_sort;
pub use self::tree_sort::tree_sort;
pub use self::wave_sort::wave_sort;
pub use self::wiggle_sort::wiggle_sort;

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
