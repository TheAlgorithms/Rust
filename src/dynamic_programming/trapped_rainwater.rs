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
pub fn trapped_rainwater(elevation_map: &[u32]) -> u32 {
    if elevation_map.is_empty() {
        return 0;
    }

    let left_max = calculate_max(elevation_map, true);
    let right_max = calculate_max(elevation_map, false);
    let mut water_trapped = 0;

    // Calculate trapped water
    for i in 0..elevation_map.len() {
        water_trapped += left_max[i].min(right_max[i]) - elevation_map[i];
    }

    water_trapped
}

/// Calculates the maximum array of the given elevation map based on the direction.
///
/// # Arguments
///
/// * `elevation_map` - A reference to a slice representing the elevation map where each element
///   represents the height of the terrain at that position.
/// * `direction` - A boolean indicating whether to calculate the maximum array from left to right (`true`)
///   or from right to left (`false`).
///
/// # Returns
///
/// A vector representing the maximum array.
fn calculate_max(elevation_map: &[u32], direction: bool) -> Vec<u32> {
    let mut max_array = vec![0; elevation_map.len()];
    match direction {
        true => {
            max_array[0] = elevation_map[0];
            for i in 1..elevation_map.len() {
                max_array[i] = max_array[i - 1].max(elevation_map[i]);
            }
        }
        false => {
            max_array[elevation_map.len() - 1] = elevation_map[elevation_map.len() - 1];
            for i in (0..(elevation_map.len() - 1)).rev() {
                max_array[i] = max_array[i + 1].max(elevation_map[i]);
            }
        }
    }
    max_array
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
                    assert_eq!(trapped_rainwater(&elevation_map), expected_trapped_water);
                }
            )*
        };
    }

    trapped_rainwater_tests! {
        test_trapped_rainwater_basic: (
            [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],
            6
        ),
        test_bucket: (
            [5, 1, 5],
            4
        ),
        test_skewed_bucket: (
            [4, 1, 5],
            3
        ),
        test_trapped_rainwater_empty: (
            [],
            0
        ),
        test_trapped_rainwater_flat: (
            [0, 0, 0, 0, 0],
            0
        ),
        test_trapped_rainwater_no_trapped_water: (
            [1, 1, 2, 4, 0, 0, 0],
            0
        ),
        test_trapped_rainwater_single_elevation_map: (
            [5],
            0
        ),
        test_trapped_rainwater_large_elevation_map_difference: (
            [5, 1, 6, 1, 7, 1, 8],
            15
        ),
    }
}
