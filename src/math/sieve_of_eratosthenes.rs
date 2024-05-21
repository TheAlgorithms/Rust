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

        // Mark non-prime numbers in the sieve and collect primes up to `end`
        update_sieve(&mut sieve, end, num, &mut result);

        // Collect remaining primes beyond `end`
        result.extend(extract_remaining_primes(&sieve, end + 1));
    }
    result
}

/// Marks non-prime numbers in the sieve and collects prime numbers up to `end`.
///
/// # Arguments
///
/// * `sieve` - A mutable slice of booleans representing the sieve.
/// * `end` - The square root of the upper limit, used to optimize the algorithm.
/// * `num` - The upper limit up to which to mark non-prime numbers.
/// * `result` - A mutable vector to store the prime numbers.
fn update_sieve(sieve: &mut [bool], end: usize, num: usize, result: &mut Vec<usize>) {
    for start in 2..=end {
        if sieve[start] {
            result.push(start); // Collect prime numbers up to `end`
            for i in (start * start..=num).step_by(start) {
                sieve[i] = false;
            }
        }
    }
}

/// Extracts remaining prime numbers from the sieve beyond the given start index.
///
/// # Arguments
///
/// * `sieve` - A slice of booleans representing the sieve with non-prime numbers marked as false.
/// * `start` - The index to start checking for primes (inclusive).
///
/// # Returns
///
/// A vector containing all remaining prime numbers extracted from the sieve.
fn extract_remaining_primes(sieve: &[bool], start: usize) -> Vec<usize> {
    sieve[start..]
        .iter()
        .enumerate()
        .filter_map(|(i, &is_prime)| if is_prime { Some(start + i) } else { None })
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
