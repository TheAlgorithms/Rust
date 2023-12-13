// Author : cyrixninja
// Calculate the Euclidean distance between two vectors
// Wikipedia : https://en.wikipedia.org/wiki/Euclidean_distance

pub fn euclidean_distance(vector_1: &Vector, vector_2: &Vector) -> f64 {
    // Calculate the Euclidean distance using the provided vectors.
    let squared_sum: f64 = vector_1
        .iter()
        .zip(vector_2.iter())
        .map(|(&a, &b)| (a - b).powi(2))
        .sum();

    squared_sum.sqrt()
}

type Vector = Vec<f64>;

#[cfg(test)]
mod tests {
    use super::*;

    // Define a test function for the euclidean_distance function.
    #[test]
    fn test_euclidean_distance() {
        // First test case: 2D vectors
        let vec1_2d = vec![1.0, 2.0];
        let vec2_2d = vec![4.0, 6.0];

        // Calculate the Euclidean distance
        let result_2d = euclidean_distance(&vec1_2d, &vec2_2d);
        assert_eq!(result_2d, 5.0);

        // Second test case: 4D vectors
        let vec1_4d = vec![1.0, 2.0, 3.0, 4.0];
        let vec2_4d = vec![5.0, 6.0, 7.0, 8.0];

        // Calculate the Euclidean distance
        let result_4d = euclidean_distance(&vec1_4d, &vec2_4d);
        assert_eq!(result_4d, 8.0);
    }
}
