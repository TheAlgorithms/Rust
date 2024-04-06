//! Module to calculate trapped rainwater in an elevation map.

/// Calculates the total amount of trapped rainwater in the given elevation map.
///
/// # Arguments
///
/// * `height` - A vector containing the heights of walls forming the elevation map.
///
/// # Returns
///
/// The total amount of trapped rainwater.
pub fn trapped_rainwater(height: Vec<u32>) -> u32 {
    if height.is_empty() {
        return 0;
    }

    let mut left_max = vec![0; height.len()];
    let mut right_max = vec![0; height.len()];
    let mut water_trapped = 0;

    // Calculate left_max array
    left_max[0] = height[0];
    for i in 1..height.len() {
        left_max[i] = left_max[i - 1].max(height[i]);
    }

    // Calculate right_max array
    right_max[height.len() - 1] = height[height.len() - 1];
    for i in (0..(height.len() - 1)).rev() {
        right_max[i] = right_max[i + 1].max(height[i]);
    }

    // Calculate trapped water
    for i in 0..height.len() {
        water_trapped += left_max[i].min(right_max[i]) - height[i];
    }

    water_trapped
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! trapped_rainwater_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (height, expected_trapped_water) = $test_case;
                    assert_eq!(trapped_rainwater(height), expected_trapped_water);
                }
            )*
        };
    }

    trapped_rainwater_tests! {
        test_trapped_rainwater_basic: (
            vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],
            6
        ),
        test_trapped_rainwater_empty: (
            Vec::new(),
            0
        ),
        test_trapped_rainwater_flat: (
            vec![0, 0, 0, 0, 0],
            0
        ),
        test_trapped_rainwater_no_trapped_water: (
            vec![1, 1, 2, 4, 0, 0, 0],
            0
        ),
        test_trapped_rainwater_single_height: (
            vec![5],
            0
        ),
        test_trapped_rainwater_large_height_difference: (
            vec![5, 1, 6, 1, 7, 1, 8],
            15
        ),
    }
}
