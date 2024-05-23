//! Module to calculate trapped rainwater in an elevation map.

/// Computes the total volume of trapped rainwater in a given elevation map.
///
/// # Arguments
///
/// * `elevation_map` - A slice containing the heights of the terrain elevations.
///
/// # Returns
///
/// The total volume of trapped rainwater.
pub fn trapped_rainwater(elevation_map: &[u32]) -> u32 {
    let left_max = calculate_max_values(elevation_map, false);
    let right_max = calculate_max_values(elevation_map, true);
    let mut water_trapped = 0;
    // Calculate trapped water
    for i in 0..elevation_map.len() {
        water_trapped += left_max[i].min(right_max[i]) - elevation_map[i];
    }
    water_trapped
}

/// Determines the maximum heights from either direction in the elevation map.
///
/// # Arguments
///
/// * `elevation_map` - A slice representing the heights of the terrain elevations.
/// * `reverse` - A boolean that indicates the direction of calculation.
///   - `false` for left-to-right.
///   - `true` for right-to-left.
///
/// # Returns
///
/// A vector containing the maximum heights encountered up to each position.
fn calculate_max_values(elevation_map: &[u32], reverse: bool) -> Vec<u32> {
    let mut max_values = vec![0; elevation_map.len()];
    let mut current_max = 0;
    for i in create_iter(elevation_map.len(), reverse) {
        current_max = current_max.max(elevation_map[i]);
        max_values[i] = current_max;
    }
    max_values
}

/// Creates an iterator for the given length, optionally reversing it.
///
/// # Arguments
///
/// * `len` - The length of the iterator.
/// * `reverse` - A boolean that determines the order of iteration.
///   - `false` for forward iteration.
///   - `true` for reverse iteration.
///
/// # Returns
///
/// A boxed iterator that iterates over the range of indices.
fn create_iter(len: usize, reverse: bool) -> Box<dyn Iterator<Item = usize>> {
    if reverse {
        Box::new((0..len).rev())
    } else {
        Box::new(0..len)
    }
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
                    let elevation_map_rev: Vec<u32> = elevation_map.iter().rev().cloned().collect();
                    assert_eq!(trapped_rainwater(&elevation_map_rev), expected_trapped_water);
                }
            )*
        };
    }

    trapped_rainwater_tests! {
        test_trapped_rainwater_basic: (
            [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],
            6
        ),
        test_trapped_rainwater_peak_under_water: (
            [3, 0, 2, 0, 4],
            7,
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
        test_trapped_rainwater_two_point_elevation_map: (
            [5, 1],
            0
        ),
        test_trapped_rainwater_large_elevation_map_difference: (
            [5, 1, 6, 1, 7, 1, 8],
            15
        ),
    }
}
