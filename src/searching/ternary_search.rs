//! This module provides an implementation of a ternary search algorithm that
//! works for both ascending and descending ordered arrays. The ternary search
//! function returns the index of the target element if it is found, or `None`
//! if the target is not present in the array.

use std::cmp::Ordering;

/// Performs a ternary search for a specified item within a sorted array.
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
///
/// # Returns
///
/// An `Option<usize>` which is:
/// - `Some(index)` if the item is found at the given index.
/// - `None` if the item is not found in the array.
pub fn ternary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let is_asc = is_asc_arr(arr);
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        if match_compare(item, arr, &mut left, &mut right, is_asc) {
            return Some(left);
        }
    }

    None
}

/// Compares the item with two middle elements of the current search range and
/// updates the search bounds accordingly. This function handles both ascending
/// and descending ordered arrays. It calculates two middle indices of the
/// current search range and compares the item with the elements at these
/// indices. It then updates the search bounds (`left` and `right`) based on
/// the result of these comparisons. If the item is found, it returns `true`.
///
/// # Parameters
///
/// - `item`: A reference to the item to search for.
/// - `arr`: A slice of the array in which to search.
/// - `left`: A mutable reference to the left bound of the search range.
/// - `right`: A mutable reference to the right bound of the search range.
/// - `is_asc`: A boolean indicating whether the array is sorted in ascending order.
///
/// # Returns
///
/// A `bool` indicating:
/// - `true` if the item was found in the array.
/// - `false` if the item was not found in the array.
fn match_compare<T: Ord>(
    item: &T,
    arr: &[T],
    left: &mut usize,
    right: &mut usize,
    is_asc: bool,
) -> bool {
    let first_mid = *left + (*right - *left) / 3;
    let second_mid = *right - (*right - *left) / 3;

    // Handling the edge case where the search narrows down to a single element
    if first_mid == second_mid && first_mid == *left {
        return match &arr[*left] {
            x if x == item => true,
            _ => {
                *left += 1;
                false
            }
        };
    }

    let cmp_first_mid = item.cmp(&arr[first_mid]);
    let cmp_second_mid = item.cmp(&arr[second_mid]);

    match (is_asc, cmp_first_mid, cmp_second_mid) {
        // If the item matches either midpoint, it returns the index
        (_, Ordering::Equal, _) => {
            *left = first_mid;
            return true;
        }
        (_, _, Ordering::Equal) => {
            *left = second_mid;
            return true;
        }
        // If the item is smaller than the element at first_mid (in ascending order)
        // or greater than it (in descending order), it narrows the search to the first third.
        (true, Ordering::Less, _) | (false, Ordering::Greater, _) => {
            *right = first_mid.saturating_sub(1)
        }
        // If the item is greater than the element at second_mid (in ascending order)
        // or smaller than it (in descending order), it narrows the search to the last third.
        (true, _, Ordering::Greater) | (false, _, Ordering::Less) => *left = second_mid + 1,
        // Otherwise, it searches the middle third.
        (_, _, _) => {
            *left = first_mid + 1;
            *right = second_mid - 1;
        }
    }

    false
}

/// Determines if the given array is sorted in ascending order.
///
/// This helper function checks if the first element of the array is less than the
/// last element, indicating an ascending order. It returns `false` if the array
/// has fewer than two elements.
///
/// # Parameters
///
/// - `arr`: A slice of the array to check.
///
/// # Returns
///
/// A `bool` indicating whether the array is sorted in ascending order.
fn is_asc_arr<T: Ord>(arr: &[T]) -> bool {
    arr.len() > 1 && arr[0] < arr[arr.len() - 1]
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
                    if let Some(expected_index) = expected {
                        assert_eq!(arr[expected_index], item);
                    }
                    assert_eq!(ternary_search(&item, arr), expected);
                }
            )*
        };
    }

    test_cases! {
        empty: ("a", &[] as &[&str], None),
        one_item_found: ("a", &["a"], Some(0)),
        one_item_not_found: ("b", &["a"], None),
        search_two_elements_found_at_start: (1, &[1, 2], Some(0)),
        search_two_elements_found_at_end: (2, &[1, 2], Some(1)),
        search_two_elements_not_found_start: (0, &[1, 2], None),
        search_two_elements_not_found_end: (3, &[1, 2], None),
        search_three_elements_found_start: (1, &[1, 2, 3], Some(0)),
        search_three_elements_found_middle: (2, &[1, 2, 3], Some(1)),
        search_three_elements_found_end: (3, &[1, 2, 3], Some(2)),
        search_three_elements_not_found_start: (0, &[1, 2, 3], None),
        search_three_elements_not_found_end: (4, &[1, 2, 3], None),
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
