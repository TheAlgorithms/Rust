/// Implements the Sieve of Eratosthenes algorithm to find all prime numbers up to a given limit.
///
/// # Arguments
///
/// * `num` - The upper limit up to which to find prime numbers (inclusive).
///
/// # Returns
///
/// A vector containing all prime numbers up to the specified limit.
pub fn sieve_of_eratosthenes(num: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    if num >= 2 {
        let mut sieve: Vec<bool> = vec![true; num + 1];

        // 0 and 1 are not prime numbers
        sieve[0] = false;
        sieve[1] = false;

        let end: usize = (num as f64).sqrt() as usize;

        // mark non-prime numbers in the sieve
        update_sieve(&mut sieve, end, num);

        // collect all prime numbers
        result = extract_primes(&sieve);
    }
    result
}

/// Marks non-prime numbers in the sieve.
///
/// # Arguments
///
/// * `sieve` - A mut slice of booleans representing the sieve.
/// * `end` - The square root of the upper limit, used to optimize the algorithm.
/// * `num` - The upper limit up to which to mark non-prime numbers.
fn update_sieve(sieve: &mut [bool], end: usize, num: usize) {
    for start in 2..=end {
        if sieve[start] {
            for i in (start * start..=num).step_by(start) {
                sieve[i] = false;
            }
        }
    }
}

/// Extracts prime numbers from the sieve.
///
/// # Arguments
///
/// * `sieve` - A slice of booleans representing the sieve with non-prime numbers marked as false.
///
/// # Returns
///
/// A vector containing all prime numbers extracted from the sieve.
fn extract_primes(sieve: &[bool]) -> Vec<usize> {
    sieve
        .into_iter()
        .enumerate()
        .filter_map(|(num, is_prime)| if *is_prime { Some(num) } else { None })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! sieve_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $test_case;
                    assert_eq!(sieve_of_eratosthenes(input), expected);
                }
            )*
        }
    }

    sieve_tests! {
        test_0: (0, Vec::<usize>::new()),
        test_11: (11, vec![2, 3, 5, 7, 11]),
        test_25: (25, vec![2, 3, 5, 7, 11, 13, 17, 19, 23]),
        test_33: (33, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]),
        test_100: (100, vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97
        ]),
    }
}
