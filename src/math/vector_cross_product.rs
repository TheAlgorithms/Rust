/// Cross Product and Magnitude Calculation
///
/// This program defines functions to calculate the cross product of two 3D vectors
/// and the magnitude of a vector from its direction ratios. The main purpose is
/// to demonstrate the mathematical concepts and provide test cases for the functions.
///
/// Time Complexity:
/// - Calculating the cross product and magnitude of a vector each takes O(1) time
///   since we are working with fixed-size arrays and performing a fixed number of
///   mathematical operations.

/// Function to calculate the cross product of two vectors
pub fn cross_product(vec1: [f64; 3], vec2: [f64; 3]) -> [f64; 3] {
    let x = vec1[1] * vec2[2] - vec1[2] * vec2[1];
    let y = -(vec1[0] * vec2[2] - vec1[2] * vec2[0]);
    let z = vec1[0] * vec2[1] - vec1[1] * vec2[0];
    [x, y, z]
}

/// Function to calculate the magnitude of a vector
pub fn vector_magnitude(vec: [f64; 3]) -> f64 {
    (vec[0].powi(2) + vec[1].powi(2) + vec[2].powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_product_and_magnitude_1() {
        // Test case with non-trivial vectors
        let vec1 = [1.0, 2.0, 3.0];
        let vec2 = [4.0, 5.0, 6.0];

        let cross_product = cross_product(vec1, vec2);
        let magnitude = vector_magnitude(cross_product);

        // Check the expected results with a tolerance for floating-point comparisons
        assert_eq!(cross_product, [-3.0, 6.0, -3.0]);
        assert!((magnitude - 7.34847).abs() < 1e-5);
    }

    #[test]
    fn test_cross_product_and_magnitude_2() {
        // Test case with orthogonal vectors
        let vec1 = [1.0, 0.0, 0.0];
        let vec2 = [0.0, 1.0, 0.0];

        let cross_product = cross_product(vec1, vec2);
        let magnitude = vector_magnitude(cross_product);

        // Check the expected results
        assert_eq!(cross_product, [0.0, 0.0, 1.0]);
        assert_eq!(magnitude, 1.0);
    }

    #[test]
    fn test_cross_product_and_magnitude_3() {
        // Test case with vectors along the axes
        let vec1 = [2.0, 0.0, 0.0];
        let vec2 = [0.0, 3.0, 0.0];

        let cross_product = cross_product(vec1, vec2);
        let magnitude = vector_magnitude(cross_product);

        // Check the expected results
        assert_eq!(cross_product, [0.0, 0.0, 6.0]);
        assert_eq!(magnitude, 6.0);
    }

    #[test]
    fn test_cross_product_and_magnitude_4() {
        // Test case with parallel vectors
        let vec1 = [1.0, 2.0, 3.0];
        let vec2 = [2.0, 4.0, 6.0];

        let cross_product = cross_product(vec1, vec2);
        let magnitude = vector_magnitude(cross_product);

        // Check the expected results
        assert_eq!(cross_product, [0.0, 0.0, 0.0]);
        assert_eq!(magnitude, 0.0);
    }

    #[test]
    fn test_cross_product_and_magnitude_5() {
        // Test case with zero vectors
        let vec1 = [0.0, 0.0, 0.0];
        let vec2 = [0.0, 0.0, 0.0];

        let cross_product = cross_product(vec1, vec2);
        let magnitude = vector_magnitude(cross_product);

        // Check the expected results
        assert_eq!(cross_product, [0.0, 0.0, 0.0]);
        assert_eq!(magnitude, 0.0);
    }
}
