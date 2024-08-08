/// Implementation of Floyd's Cycle Detection algorithm -> Hare and Tortoise algorithm
/// Given an array of integers containing 'n + 1' integers, where each
/// integer is in the range [1, n] inclusive. If there is only one duplicate
/// number in the input array, this algorithm returns the duplicate number in
/// O(1) space and the time complexity is less than O(n^2) without modifying the
/// original array, otherwise, it returns -1.
///
/// Author: Tashvik (https://github.com/tashviks)

/// Function to find the duplicate number using Floyd's Cycle Detection algorithm
fn find_duplicate_number(nums: &[u32]) -> Option<u32> {
    if nums.is_empty() {
        return None; // No duplicates if array is empty
    }

    let mut tortoise = nums[0];
    let mut hare = nums[0];

    // Phase 1: Finding the intersection point
    loop {
        tortoise = nums[tortoise as usize];
        hare = nums[nums[hare as usize] as usize];

        if tortoise == hare {
            break;
        }
    }

    // Phase 2: Finding the duplicate number
    tortoise = nums[0];
    while tortoise != hare {
        tortoise = nums[tortoise as usize];
        hare = nums[hare as usize];
    }

    Some(tortoise)
}

fn main() {
    // Test cases
    let array1 = vec![3, 4, 8, 5, 9, 1, 2, 6, 7, 4];
    println!("Array 1: {:?}", find_duplicate_number(&array1)); // Outputs: Some(4)

    let array2 = vec![1, 2, 3, 4, 2];
    println!("Array 2: {:?}", find_duplicate_number(&array2)); // Outputs: Some(2)

    let array3: Vec<u32> = vec![];
    println!("Array 3: {:?}", find_duplicate_number(&array3)); // Outputs: None
}
