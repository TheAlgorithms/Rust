// Author : cyrixninja
// Find the Sum of Geometric Progression
// Wikipedia: https://en.wikipedia.org/wiki/Geometric_progression

pub fn sum_of_geometric_progression(first_term: f64, common_ratio: f64, num_of_terms: i32) -> f64 {
    if common_ratio == 1.0 {
        // Formula for sum if the common ratio is 1
        return (num_of_terms as f64) * first_term;
    }

    // Formula for finding the sum of n terms of a Geometric Progression
    return (first_term / (1.0 - common_ratio)) * (1.0 - common_ratio.powi(num_of_terms));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_geometric_progression() {
        let result_1 = sum_of_geometric_progression(1.0, 2.0, 10);
        assert_eq!(result_1, 1023.0);

        let result_2 = sum_of_geometric_progression(1.0, 10.0, 5);
        assert_eq!(result_2, 11111.0);

        let result_3 = sum_of_geometric_progression(9.0, 2.5, 5);
        assert_eq!(result_3, 579.9375);

    }
}
