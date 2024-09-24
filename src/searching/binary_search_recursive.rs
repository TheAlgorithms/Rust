use std::cmp::Ordering;

/// Recursively performs a binary search for a specified item within a sorted array.
///
/// This function can handle both ascending and descending ordered arrays. It
/// takes a reference to the item to search for and a slice of the array. If
/// the item is found, it returns the index of the item within the array. If
/// the item is not found, it returns `None`.
///
/// # Parameters
///
/// - `item`: A reference to the item to search for.
/// - `arr`: A slice of the sorted array in which to search.
/// - `left`: The left bound of the current search range.
/// - `right`: The right bound of the current search range.
/// - `is_asc`: A boolean indicating whether the array is sorted in ascending order.
///
/// # Returns
///
/// An `Option<usize>` which is:
/// - `Some(index)` if the item is found at the given index.
/// - `None` if the item is not found in the array.
pub fn binary_search_rec<T: Ord>(item: &T, arr: &[T], left: usize, right: usize) -> Option<usize> {
    if left >= right {
        return None;
    }

    let is_asc = arr.len() > 1 && arr[0] < arr[arr.len() - 1];
    let mid = left + (right - left) / 2;
    let cmp_result = item.cmp(&arr[mid]);

    match (is_asc, cmp_result) {
        (true, Ordering::Less) | (false, Ordering::Greater) => {
            binary_search_rec(item, arr, left, mid)
        }
        (true, Ordering::Greater) | (false, Ordering::Less) => {
            binary_search_rec(item, arr, mid + 1, right)
        }
        (_, Ordering::Equal) => Some(mid),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_cases {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (item, arr, expected) = $test_case;
                    assert_eq!(binary_search_rec(&item, arr, 0, arr.len()), expected);
                }
            )*
        };
    }

    test_cases! {
        empty: ("a", &[] as &[&str], None),
        one_item_found: ("a", &["a"], Some(0)),
        one_item_not_found: ("b", &["a"], None),
        search_strings_asc_start: ("a", &["a", "b", "c", "d", "google", "zoo"], Some(0)),
        search_strings_asc_middle: ("google", &["a", "b", "c", "d", "google", "zoo"], Some(4)),
        search_strings_asc_last: ("zoo", &["a", "b", "c", "d", "google", "zoo"], Some(5)),
        search_strings_asc_not_found: ("x", &["a", "b", "c", "d", "google", "zoo"], None),
        search_strings_desc_start: ("zoo", &["zoo", "google", "d", "c", "b", "a"], Some(0)),
        search_strings_desc_middle: ("google", &["zoo", "google", "d", "c", "b", "a"], Some(1)),
        search_strings_desc_last: ("a", &["zoo", "google", "d", "c", "b", "a"], Some(5)),
        search_strings_desc_not_found: ("x", &["zoo", "google", "d", "c", "b", "a"], None),
        search_ints_asc_start: (1, &[1, 2, 3, 4], Some(0)),
        search_ints_asc_middle: (3, &[1, 2, 3, 4], Some(2)),
        search_ints_asc_end: (4, &[1, 2, 3, 4], Some(3)),
        search_ints_asc_not_found: (5, &[1, 2, 3, 4], None),
        search_ints_desc_start: (4, &[4, 3, 2, 1], Some(0)),
        search_ints_desc_middle: (3, &[4, 3, 2, 1], Some(1)),
        search_ints_desc_end: (1, &[4, 3, 2, 1], Some(3)),
        search_ints_desc_not_found: (5, &[4, 3, 2, 1], None),
        with_gaps_0: (0, &[1, 3, 8, 11], None),
        with_gaps_1: (1, &[1, 3, 8, 11], Some(0)),
        with_gaps_2: (2, &[1, 3, 8, 11], None),
        with_gaps_3: (3, &[1, 3, 8, 11], Some(1)),
        with_gaps_4: (4, &[1, 3, 8, 10], None),
        with_gaps_5: (5, &[1, 3, 8, 10], None),
        with_gaps_6: (6, &[1, 3, 8, 10], None),
        with_gaps_7: (7, &[1, 3, 8, 11], None),
        with_gaps_8: (8, &[1, 3, 8, 11], Some(2)),
        with_gaps_9: (9, &[1, 3, 8, 11], None),
        with_gaps_10: (10, &[1, 3, 8, 11], None),
        with_gaps_11: (11, &[1, 3, 8, 11], Some(3)),
        with_gaps_12: (12, &[1, 3, 8, 11], None),
        with_gaps_13: (13, &[1, 3, 8, 11], None),
    }
}
