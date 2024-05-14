//! Module to calculate trapped rainwater in an elevation map.

/// Calculates the total amount of trapped rainwater in the given elevation map.
///
/// # Arguments
///
/// * `elevation_map` - A vector containing the heights of walls forming the elevation map.
///
/// # Returns
///
/// The total amount of trapped rainwater.
pub fn trapped_rainwater(elevation_map: Vec<u32>) -> u32 {
    if elevation_map.is_empty() {
        return 0;
    }

    let mut left_max = vec![0; elevation_map.len()];
    let mut right_max = vec![0; elevation_map.len()];
    let mut water_trapped = 0;

    // Calculate left_max array
    left_max[0] = elevation_map[0];
    for i in 1..elevation_map.len() {
        left_max[i] = left_max[i - 1].max(elevation_map[i]);
    }

    // Calculate right_max array
    right_max[elevation_map.len() - 1] = elevation_map[elevation_map.len() - 1];
    for i in (0..(elevation_map.len() - 1)).rev() {
        right_max[i] = right_max[i + 1].max(elevation_map[i]);
    }

    // Calculate trapped water
    for i in 0..elevation_map.len() {
        water_trapped += left_max[i].min(right_max[i]) - elevation_map[i];
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
                    let (elevation_map, expected_trapped_water) = $test_case;
                    assert_eq!(trapped_rainwater(elevation_map), expected_trapped_water);
                }
            )*
        };
    }

    trapped_rainwater_tests! {
        test_trapped_rainwater_basic: (
            vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],
            6
        ),
        test_bucket: (
            vec![5, 1, 5],
            4
        ),
        test_skewed_bucket: (
            vec![4, 1, 5],
            3
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
        test_trapped_rainwater_single_elevation_map: (
            vec![5],
            0
        ),
        test_trapped_rainwater_large_elevation_map_difference: (
            vec![5, 1, 6, 1, 7, 1, 8],
            15
        ),
    }
}
