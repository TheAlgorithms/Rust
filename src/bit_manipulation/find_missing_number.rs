/// Finds the missing number in a slice of consecutive integers.
///
/// This function uses XOR bitwise operation to find the missing number.
/// It XORs all expected numbers in the range [min, max] with the actual
/// numbers present in the array. Since XOR has the property that `a ^ a = 0`,
/// all present numbers cancel out, leaving only the missing number.
///
/// # Arguments
///
/// * `nums` - A slice of integers forming a sequence with one missing number
///
/// # Returns
///
/// * `Ok(i32)` - The missing number in the sequence
/// * `Err(String)` - An error message if the input is invalid
///
/// # Examples
///
/// ```
/// # use the_algorithms_rust::bit_manipulation::find_missing_number;
/// assert_eq!(find_missing_number(&[0, 1, 3, 4]).unwrap(), 2);
/// assert_eq!(find_missing_number(&[4, 3, 1, 0]).unwrap(), 2);
/// assert_eq!(find_missing_number(&[-4, -3, -1, 0]).unwrap(), -2);
/// assert_eq!(find_missing_number(&[-2, 2, 1, 3, 0]).unwrap(), -1);
/// assert_eq!(find_missing_number(&[1, 3, 4, 5, 6]).unwrap(), 2);
/// ```
pub fn find_missing_number(nums: &[i32]) -> Result<i32, String> {
    if nums.is_empty() {
        return Err("input array must not be empty".to_string());
    }

    if nums.len() == 1 {
        return Err("array must have at least 2 elements to find a missing number".to_string());
    }

    let low = *nums.iter().min().unwrap();
    let high = *nums.iter().max().unwrap();

    let mut missing_number = high;

    for i in low..high {
        let index = (i - low) as usize;
        missing_number ^= i ^ nums[index];
    }

    Ok(missing_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_in_middle() {
        assert_eq!(find_missing_number(&[0, 1, 3, 4]).unwrap(), 2);
    }

    #[test]
    fn test_unordered_array() {
        assert_eq!(find_missing_number(&[4, 3, 1, 0]).unwrap(), 2);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(find_missing_number(&[-4, -3, -1, 0]).unwrap(), -2);
    }

    #[test]
    fn test_negative_and_positive() {
        assert_eq!(find_missing_number(&[-2, 2, 1, 3, 0]).unwrap(), -1);
    }

    #[test]
    fn test_missing_at_start() {
        assert_eq!(find_missing_number(&[1, 3, 4, 5, 6]).unwrap(), 2);
    }

    #[test]
    fn test_unordered_missing_middle() {
        assert_eq!(find_missing_number(&[6, 5, 4, 2, 1]).unwrap(), 3);
    }

    #[test]
    fn test_another_unordered() {
        assert_eq!(find_missing_number(&[6, 1, 5, 3, 4]).unwrap(), 2);
    }

    #[test]
    fn test_empty_array() {
        assert!(find_missing_number(&[]).is_err());
        assert_eq!(
            find_missing_number(&[]).unwrap_err(),
            "input array must not be empty"
        );
    }

    #[test]
    fn test_single_element() {
        assert!(find_missing_number(&[5]).is_err());
        assert_eq!(
            find_missing_number(&[5]).unwrap_err(),
            "array must have at least 2 elements to find a missing number"
        );
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(find_missing_number(&[0, 2]).unwrap(), 1);
        assert_eq!(find_missing_number(&[2, 0]).unwrap(), 1);
    }

    #[test]
    fn test_large_range() {
        assert_eq!(find_missing_number(&[100, 101, 103, 104]).unwrap(), 102);
    }

    #[test]
    fn test_missing_at_boundaries() {
        // Missing is the second to last element
        assert_eq!(find_missing_number(&[1, 2, 3, 5]).unwrap(), 4);
    }
}
