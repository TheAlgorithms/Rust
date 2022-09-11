// Operations based around amicable numbers
// Suports i32 but should be interchangable with other types
// Wikipedia reference: https://en.wikipedia.org/wiki/Amicable_numbers

// Returns vec of amicable pairs below N
// N must be positive
pub fn amicable_pairs_under_n(n: u32) -> Vec<(i32, i32)> {
    let mut factor_sums = vec![0; n as usize];

    // Make a list of the sum of the factors of each number below N
    for i in 1..n {
        for j in (i*2..n).step_by(i as usize) {
            factor_sums[j as usize] += i;
        }
    }

    // Default return value of (0, 0) if no pairs are found
    let mut out = vec![(0, 0)];
    // Check if numbers are amicable then append
    for (i, x) in factor_sums.iter().enumerate() {
        if (*x != i as u32) && (*x < n) && (factor_sums[*x as usize] == i as u32) && (*x > i as u32) {
            out.push((i as i32, *x as i32));
        }
    }

    // Check if anything was added to the vec, if so remove the (0, 0)
    if out.len() != 1 {
        out.remove(0);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::amicable_pairs_under_n;

    #[test]
    fn test_amicable_pairs_under_n() {
        // First 10 amicable numbers
        let expected_result = vec![
            (220, 284),
            (1184, 1210),
            (2620, 2924),
            (5020, 5564),
            (6232, 6368),
            (10744, 10856),
            (12285, 14595),
            (17296, 18416),
            (63020, 76084),
            (66928, 66992),
        ];
        
        // Generate pairs under 100,000
        let mut result = amicable_pairs_under_n(100_000);
        
        // There should be 13 pairs under 100,000
        assert_eq!(result.len(), 13);

        // Check the first 10 against known values
        result = result[0..10].to_vec();
        assert_eq!(result, expected_result);
        
        // For a number too small, the result should be a vec of (0, 0) --> note this could be an
        // Option if you wanted to bring in a dependency
        result = amicable_pairs_under_n(100);
        assert_eq!(result, vec![(0, 0)])
    }
}
