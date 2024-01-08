pub fn perfect_cube(n: i32) -> bool {
    // Calculate the cube root using floating-point arithmetic.
    let val = (n as f64).powf(1.0 / 3.0);
    // Check if the cube of the cube root equals the original number.
    (val * val * val) == (n as f64)
}

// Check if a number is a perfect cube using binary search.
pub fn perfect_cube_binary_search(n: i64) -> bool {
    // Handle negative numbers, as cube roots are not defined for negatives.
    if n < 0 {
        return false;
    }

    // Initialize left and right boundaries for binary search.
    let mut left = 0;
    let mut right = n.abs(); // Use the absolute value to handle negative numbers

    // Binary search loop to find the cube root.
    while left <= right {
        // Calculate the mid-point.
        let mid = left + (right - left) / 2;
        // Calculate the cube of the mid-point.
        let cube = mid * mid * mid;

        // Check if the cube equals the original number.
        match cube.cmp(&n) {
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid - 1,
        }
    }

    // If no cube root is found, return false.
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_cube() {
        assert!(perfect_cube_binary_search(27));
        assert!(!perfect_cube_binary_search(4));
    }

    #[test]
    fn test_perfect_cube_binary_search() {
        assert!(perfect_cube_binary_search(27));
        assert!(perfect_cube_binary_search(64));
        assert!(!perfect_cube_binary_search(4));
        assert!(!perfect_cube_binary_search(-8));
    }
}
