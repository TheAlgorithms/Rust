//Wiggle Sort.
//Given an unsorted array nums, reorder it such
//that nums[0] < nums[1] > nums[2] < nums[3]....
//For example:
//if input numbers = [3, 5, 2, 1, 6, 4]
//one possible Wiggle Sorted answer is [3, 5, 1, 6, 2, 4].

pub fn wiggle_sort(nums: Vec<i32>) -> Vec<i32> {
//Rust implementation of wiggle.
//    Example:
//    >>> wiggle_sort([0, 5, 3, 2, 2])
//    [0, 5, 2, 3, 2]
//    >>> wiggle_sort([])
//    []
//    >>> wiggle_sort([-2, -5, -45])
//    [-45, -2, -5]

    let len = nums.len();
    let mut p = nums;
    for i in 1..len {
        let num_x = p[i - 1];
        let num_y = p[i];
        if (i % 2 == 1) == (num_x > num_y) {
            p[i - 1] = num_y;
            p[i] = num_x;
        }
    }
    return p
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wingle_elements() {
        let arr = vec![3, 5, 2, 1, 6, 4];
        let res = wiggle_sort(arr.clone());
        assert_eq!(res, [3, 5, 1, 6, 2, 4]);
    }
    
    #[test]
    fn odd_number_of_elements() {
        let arr = vec![4, 1, 3, 5, 2];
        let res = wiggle_sort(arr.clone());
        assert_eq!(res, [1, 4, 3, 5, 2]);
    }

    #[test]
    fn repeated_elements() {
        let arr = vec![5, 5, 5, 5];
        let res = wiggle_sort(arr.clone());
        assert_eq!(res, [5, 5, 5, 5]);
    }
}
