/// Finds the unique number in a slice where every other element appears twice.
///
/// This function uses the XOR bitwise operation. Since XOR has the property that
/// `a ^ a = 0` and `a ^ 0 = a`, all paired numbers cancel out, leaving only the
/// unique number.
///
/// # Arguments
///
/// * `arr` - A slice of integers where all elements except one appear exactly twice
///
/// # Returns
///
/// * `Ok(i32)` - The unique number that appears only once
/// * `Err(String)` - An error message if the input is empty
///
/// # Examples
///
/// ```
/// # use the_algorithms_rust::bit_manipulation::find_unique_number;
/// assert_eq!(find_unique_number(&[1, 1, 2, 2, 3]).unwrap(), 3);
/// assert_eq!(find_unique_number(&[4, 5, 4, 6, 6]).unwrap(), 5);
/// assert_eq!(find_unique_number(&[7]).unwrap(), 7);
/// assert_eq!(find_unique_number(&[10, 20, 10]).unwrap(), 20);
/// assert!(find_unique_number(&[]).is_err());
/// ```
pub fn find_unique_number(arr: &[i32]) -> Result<i32, String> {
    if arr.is_empty() {
        return Err("input list must not be empty".to_string());
    }

    let result = arr.iter().fold(0, |acc, &num| acc ^ num);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        assert_eq!(find_unique_number(&[1, 1, 2, 2, 3]).unwrap(), 3);
    }

    #[test]
    fn test_different_order() {
        assert_eq!(find_unique_number(&[4, 5, 4, 6, 6]).unwrap(), 5);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(find_unique_number(&[7]).unwrap(), 7);
    }

    #[test]
    fn test_three_elements() {
        assert_eq!(find_unique_number(&[10, 20, 10]).unwrap(), 20);
    }

    #[test]
    fn test_empty_array() {
        assert!(find_unique_number(&[]).is_err());
        assert_eq!(
            find_unique_number(&[]).unwrap_err(),
            "input list must not be empty"
        );
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(find_unique_number(&[-1, -1, -2, -2, -3]).unwrap(), -3);
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(
            find_unique_number(&[1000, 2000, 1000, 3000, 3000]).unwrap(),
            2000
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(find_unique_number(&[0, 1, 1]).unwrap(), 0);
    }
}
