// Author : cyrixninja
// Sum of Harmonic Series :    Find the sum of n terms in an harmonic progression.  The calculation starts with the
//                             first_term and loops adding the common difference of Arithmetic Progression by which
//                             the given Harmonic Progression is linked.
// Wikipedia Reference  :  https://en.wikipedia.org/wiki/Interquartile_range
// Other References     :  https://the-algorithms.com/algorithm/sum-of-harmonic-series?lang=python

pub fn sum_of_harmonic_progression(
    first_term: f64,
    common_difference: f64,
    number_of_terms: i32,
) -> f64 {
    let mut arithmetic_progression = vec![1.0 / first_term];
    let mut current_term = 1.0 / first_term;

    for _ in 0..(number_of_terms - 1) {
        current_term += common_difference;
        arithmetic_progression.push(current_term);
    }

    let harmonic_series: Vec<f64> = arithmetic_progression
        .into_iter()
        .map(|step| 1.0 / step)
        .collect();
    harmonic_series.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_harmonic_progression() {
        assert_eq!(sum_of_harmonic_progression(1.0 / 2.0, 2.0, 2), 0.75);
        assert_eq!(
            sum_of_harmonic_progression(1.0 / 5.0, 5.0, 5),
            0.45666666666666667
        );
    }
}
