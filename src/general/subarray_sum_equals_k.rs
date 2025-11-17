use std::collections::HashMap;

/// Counts the number of contiguous subarrays that sum to exactly k.
///
/// # Parameters
///
/// - `nums`: A slice of integers
/// - `k`: The target sum
///
/// # Returns
///
/// The number of contiguous subarrays with sum equal to k.
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(n)

pub fn subarray_sum_equals_k(nums: &[i32], k: i32) -> i32 {
    let mut prefix_sum_count: HashMap<i64, i32> = HashMap::new();
    prefix_sum_count.insert(0, 1);

    let mut prefix_sum: i64 = 0;
    let mut count = 0;

    for &num in nums {
        prefix_sum += num as i64;
        let target = prefix_sum - k as i64;

        if let Some(&freq) = prefix_sum_count.get(&target) {
            count += freq;
        }

        *prefix_sum_count.entry(prefix_sum).or_insert(0) += 1;
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(subarray_sum_equals_k(&[1, 1, 1], 2), 2);
        assert_eq!(subarray_sum_equals_k(&[1, 2, 3], 3), 2);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(subarray_sum_equals_k(&[1], 1), 1);
        assert_eq!(subarray_sum_equals_k(&[1], 0), 0);
    }

    #[test]
    fn test_empty() {
        assert_eq!(subarray_sum_equals_k(&[], 0), 0);
        assert_eq!(subarray_sum_equals_k(&[], 5), 0);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(subarray_sum_equals_k(&[-1, -1, 1], 0), 1);
        assert_eq!(subarray_sum_equals_k(&[1, -1, 0], 0), 3);
    }

    #[test]
    fn test_no_match() {
        assert_eq!(subarray_sum_equals_k(&[1, 2, 3], 10), 0);
    }

    #[test]
    fn test_multiple_matches() {
        assert_eq!(subarray_sum_equals_k(&[1, 0, 1, 0, 1], 1), 8);
    }
}
