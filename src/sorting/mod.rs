mod bubble_sort;
mod counting_sort;
mod heap_sort;
mod insertion;
mod quick_sort;
mod selection_sort;
mod merge_sort;

use std::cmp;

pub use self::bubble_sort::bubble_sort;
pub use self::counting_sort::counting_sort;
pub use self::counting_sort::generic_counting_sort;
pub use self::heap_sort::heap_sort;
pub use self::insertion::insertion_sort;
pub use self::quick_sort::quick_sort;
pub use self::selection_sort::selection_sort;
pub use self::merge_sort::merge_sort;

pub fn is_sorted<T>(arr: &[T]) -> bool
where
    T: cmp::PartialOrd,
{
    if arr.is_empty() {
        return true;
    }

    let mut prev = &arr[0];

    for item in arr.iter().skip(1) {
        if prev > &item {
            return false;
        }

        prev = &item;
    }

    true
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

        assert_eq!(is_sorted(&[1, 0]), false);
        assert_eq!(is_sorted(&[2, 3, 1, -1, 5]), false);
    }
}
