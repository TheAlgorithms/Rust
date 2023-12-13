// Author : cyrixninja
// Find the Sum of Geometric Progression
// Wikipedia: https://en.wikipedia.org/wiki/Geometric_progression

pub fn sum_of_geometric_progression(first_term: f64, common_ratio: f64, num_of_terms: i32) -> f64 {
    if common_ratio == 1.0 {
        // Formula for sum if the common ratio is 1
        return (num_of_terms as f64) * first_term;
    }

    // Formula for finding the sum of n terms of a Geometric Progression
    (first_term / (1.0 - common_ratio)) * (1.0 - common_ratio.powi(num_of_terms))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_sum_of_geometric_progression {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (first_term, common_ratio, num_of_terms, expected) = $inputs;
                assert_eq!(sum_of_geometric_progression(first_term, common_ratio, num_of_terms), expected);
            }
        )*
        }
    }

    test_sum_of_geometric_progression! {
        regular_input_0: (1.0, 2.0, 10, 1023.0),
        regular_input_1: (1.0, 10.0, 5, 11111.0),
        regular_input_2: (9.0, 2.5, 5, 579.9375),
        common_ratio_one: (10.0, 1.0, 3, 30.0),
    }
}
