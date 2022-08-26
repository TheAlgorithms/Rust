pub fn interpolation_search<Ordering>(nums: &[i32], item: &i32) -> Result<usize, usize> {
    // early check
    if nums.is_empty() {
        return Err(0);
    }
    let mut low: usize = 0;
    let mut high: usize = nums.len() - 1;
    while low <= high {
        if *item < nums[low] || *item > nums[high] {
            break;
        }
        let offset: usize = low
            + (((high - low) / (nums[high] - nums[low]) as usize) * (*item - nums[low]) as usize);
        match nums[offset].cmp(item) {
            std::cmp::Ordering::Equal => return Ok(offset),
            std::cmp::Ordering::Less => low = offset + 1,
            std::cmp::Ordering::Greater => high = offset - 1,
        }
    }
    Err(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn returns_err_if_empty_slice() {
        let nums = [];
        assert_eq!(interpolation_search::<Ordering>(&nums, &3), Err(0));
    }

    #[test]
    fn returns_err_if_target_not_found() {
        let nums = [1, 2, 3, 4, 5, 6];
        assert_eq!(interpolation_search::<Ordering>(&nums, &10), Err(0));
    }

    #[test]
    fn returns_first_index() {
        let index: Result<usize, usize> = interpolation_search::<Ordering>(&[1, 2, 3, 4, 5], &1);
        assert_eq!(index, Ok(0));
    }

    #[test]
    fn returns_last_index() {
        let index: Result<usize, usize> = interpolation_search::<Ordering>(&[1, 2, 3, 4, 5], &5);
        assert_eq!(index, Ok(4));
    }

    #[test]
    fn returns_middle_index() {
        let index: Result<usize, usize> = interpolation_search::<Ordering>(&[1, 2, 3, 4, 5], &3);
        assert_eq!(index, Ok(2));
    }
}
