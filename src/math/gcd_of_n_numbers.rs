/// returns the greatest common divisor of n numbers
pub fn gcd(nums: &[usize]) -> usize {
    assert!(!nums.is_empty());
    let mut j = 1;
    let mut gcd = nums[0];
    while j < nums.len() {
        if nums[j] % gcd == 0 {
            j += 1;
        } else {
            gcd = nums[j] % gcd;
        }
    }
    gcd
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(gcd(&[1, 2, 3, 4, 5]), 1);
        assert_eq!(gcd(&[2, 4, 6, 8, 10]), 2);
        assert_eq!(gcd(&[3, 6, 9, 12, 15]), 3);
        assert_eq!(gcd(&[10]), 10);
    }
}
