use std::cmp::Ordering;

/// Calculates the maximum height of stacked cuboids by rearranging and stacking them.
///
/// Given a vector of cuboids represented by their dimensions (length, width, height),
/// this function calculates the maximum height of a stack of cuboids by rearranging them.
/// Cuboids can be rotated to change their dimensions to fit on top of each other.
///
/// # Arguments
///
/// * `cuboids` - A vector of cuboids represented as vectors of dimensions [length, width, height].
///
/// # Returns
///
/// The maximum height of the stacked cuboids.
///
/// # Complexity
/// Time Complexity -> O(n ^ 2)
/// Space Complexity -> O(n)
pub fn max_height_by_stacking_cuboids(cuboids: Vec<Vec<i32>>) -> i32 {
    let m = cuboids.len();

    // Step 1: Prepare the cuboids by sorting and rearranging them.
    let mut cuboids = cuboids
        .into_iter()
        .map(|mut cub| {
            cub.sort();
            cub.reverse();
            (cub[0], cub[1], cub[2])
        })
        .collect::<Vec<_>>();

    // Step 2: Sort the cuboids based on dimensions and a specific ordering criterion.
    cuboids.sort();
    cuboids.sort_by(|(x1, y1, z1), (x2, y2, z2)| {
        if x1 < x2 && y1 < y2 && z1 < z2 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    // Step 3: Initialize a vector to store maximum heights for each cuboid.
    let mut max_heights = vec![(0, 0, 0); m];

    // Step 4: Compute maximum heights iteratively.
    max_heights[0] = cuboids[0];

    for (i, (x, y, z)) in cuboids.into_iter().skip(1).enumerate() {
        for j in 0..=i {
            let prev = max_heights[j];
            if prev.1 >= y && prev.2 >= z {
                max_heights[i + 1] = max_heights[i + 1].max((x + prev.0, y, z));
            } else {
                max_heights[i + 1] = max_heights[i + 1].max((x, y, z));
            }
        }
    }

    // Step 5: Find the maximum height from the computed results.
    max_heights
        .into_iter()
        .map(|(x, _, _)| x)
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_height_case1() {
        let cuboids = vec![vec![50, 45, 20], vec![95, 37, 53], vec![45, 23, 12]];

        assert_eq!(max_height_by_stacking_cuboids(cuboids), 190);
    }

    #[test]
    fn test_max_height_case2() {
        let cuboids = vec![vec![38, 25, 45], vec![76, 35, 3]];

        assert_eq!(max_height_by_stacking_cuboids(cuboids), 76);
    }

    #[test]
    fn test_max_height_case3() {
        let cuboids = vec![
            vec![7, 11, 17],
            vec![7, 17, 11],
            vec![11, 7, 17],
            vec![11, 17, 7],
            vec![17, 7, 11],
            vec![17, 11, 7],
        ];

        assert_eq!(max_height_by_stacking_cuboids(cuboids), 102);
    }
}
