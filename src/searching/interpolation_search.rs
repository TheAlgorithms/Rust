//! Interpolation search.
//!
//! Interpolation search improves on binary search for data that is sorted
//! **and** uniformly distributed. Instead of always probing the middle of the
//! current range, it estimates where the item should be by linearly
//! interpolating between the values at the range's endpoints:
//!
//! ```text
//! offset = low + (high - low) * (item - nums[low]) / (nums[high] - nums[low])
//! ```
//!
//! On uniformly distributed input this runs in `O(log log n)` on average,
//! degrading to `O(n)` in the worst case.
//!
//! Wikipedia reference: <https://en.wikipedia.org/wiki/Interpolation_search>

use std::cmp::Ordering;

/// Searches for `item` in the ascending-sorted slice `nums`.
///
/// # Returns
///
/// * `Ok(index)` — an index at which `item` occurs. When `nums` contains
///   duplicates of `item`, any one of their indices may be returned.
/// * `Err(0)` — `item` is not present in `nums`.
pub fn interpolation_search(nums: &[i32], item: &i32) -> Result<usize, usize> {
    if nums.is_empty() {
        return Err(0);
    }

    let mut low: usize = 0;
    let mut high: usize = nums.len() - 1;

    // The loop condition doubles as the "item is out of range" check: as soon
    // as `item` falls outside `[nums[low], nums[high]]` it cannot be present.
    while low <= high && nums[low] <= *item && *item <= nums[high] {
        let low_value = i64::from(nums[low]);
        let high_value = i64::from(nums[high]);

        // A constant range cannot be interpolated (it would divide by zero).
        // The loop condition already established `nums[low] <= item <= nums[high]`,
        // so every element in the range equals `item`.
        if low_value == high_value {
            return Ok(low);
        }

        // Multiply before dividing, otherwise the quotient truncates to zero
        // whenever the value range is wider than the index range and the probe
        // degenerates into a linear scan. `i128` keeps the intermediate product
        // from overflowing for any slice length.
        let span = i128::from(high_value - low_value);
        let offset = low
            + (((high - low) as i128 * i128::from(i64::from(*item) - low_value)) / span) as usize;

        match nums[offset].cmp(item) {
            Ordering::Equal => return Ok(offset),
            Ordering::Less => low = offset + 1,
            // `nums[low] <= item` and `low <= offset`, so `nums[offset] > item`
            // implies `offset > low >= 0`; the subtraction cannot underflow.
            Ordering::Greater => high = offset - 1,
        }
    }

    Err(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_cases {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (nums, item, expected) = $test_case;
                    assert_eq!(interpolation_search(nums, &item), expected);
                }
            )*
        };
    }

    test_cases! {
        empty_slice: (&[] as &[i32], 3, Err(0)),
        item_above_range: (&[1, 2, 3, 4, 5, 6], 10, Err(0)),
        item_below_range: (&[1, 2, 3, 4, 5, 6], -10, Err(0)),
        first_index: (&[1, 2, 3, 4, 5], 1, Ok(0)),
        last_index: (&[1, 2, 3, 4, 5], 5, Ok(4)),
        middle_index: (&[1, 2, 3, 4, 5], 3, Ok(2)),
        // Regression: a single-element slice made `nums[high] - nums[low]` zero.
        single_element_found: (&[7], 7, Ok(0)),
        single_element_not_found: (&[7], 8, Err(0)),
        // Regression: every endpoint pair is equal, so every probe divided by zero.
        all_duplicates: (&[2, 2, 2], 2, Ok(0)),
        two_equal_elements: (&[5, 5], 5, Ok(0)),
        // Regression: the range converges onto one element mid-search.
        converging_range: (&[0, 1, 10], 10, Ok(2)),
        gap_between_endpoints: (&[0, 100], 50, Err(0)),
        gap_endpoint_found: (&[0, 100], 100, Ok(1)),
        // Regression: `(high - low) / (nums[high] - nums[low])` truncated to 0
        // here, turning the search into a linear scan.
        sparse_values: (&[1, 2, 3, 4, 5, 6, 7, 8, 9, 100], 9, Ok(8)),
        sparse_values_last: (&[1, 2, 3, 4, 5, 6, 7, 8, 9, 100], 100, Ok(9)),
        sparse_values_absent: (&[1, 2, 3, 4, 5, 6, 7, 8, 9, 100], 50, Err(0)),
        negative_values: (&[-20, -10, 0, 10, 20], -10, Ok(1)),
        negative_values_absent: (&[-20, -10, 0, 10, 20], -15, Err(0)),
        // Regression: `nums[high] - nums[low]` overflowed `i32`.
        extreme_range: (&[i32::MIN, 0, i32::MAX], 0, Ok(1)),
        extreme_range_low: (&[i32::MIN, 0, i32::MAX], i32::MIN, Ok(0)),
        extreme_range_high: (&[i32::MIN, 0, i32::MAX], i32::MAX, Ok(2)),
    }

    #[test]
    fn duplicates_return_a_matching_index() {
        let nums = [1, 3, 3, 3, 3, 7, 9];
        for item in [1, 3, 7, 9] {
            let index = interpolation_search(&nums, &item).unwrap();
            assert_eq!(nums[index], item);
        }
    }

    /// Exhaustively compares against a linear scan. Before the fix roughly 5%
    /// of these searches aborted with "attempt to divide by zero".
    #[test]
    fn matches_linear_scan_exhaustively() {
        let mut seed = 1u64;
        for len in 1..12usize {
            for _ in 0..200 {
                seed = seed
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let mut nums: Vec<i32> = (0..len)
                    .map(|i| ((seed >> (3 * (i % 20))) % 12) as i32)
                    .collect();
                nums.sort_unstable();

                for item in -1..13i32 {
                    match interpolation_search(&nums, &item) {
                        Ok(index) => assert_eq!(
                            nums[index], item,
                            "returned index {index} of {nums:?} does not hold {item}"
                        ),
                        Err(_) => assert!(
                            !nums.contains(&item),
                            "{item} is present in {nums:?} but was reported missing"
                        ),
                    }
                }
            }
        }
    }
}
