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

    const PRIMES_UP_TO_997: [usize; 168] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
        509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619,
        631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743,
        751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863,
        877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
    ];

    macro_rules! sieve_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let input: usize = $test_case;
                    let expected: Vec<usize> = PRIMES_UP_TO_997.iter().cloned().filter(|&x| x <= input).collect();
                    assert_eq!(sieve_of_eratosthenes(input), expected);
                }
            )*
        }
    }

    sieve_tests! {
        test_0: 0,
        test_1: 1,
        test_2: 2,
        test_3: 3,
        test_4: 4,
        test_5: 5,
        test_6: 6,
        test_7: 7,
        test_11: 11,
        test_23: 23,
        test_24: 24,
        test_25: 25,
        test_26: 26,
        test_27: 27,
        test_28: 28,
        test_29: 29,
        test_33: 33,
        test_100: 100,
        test_997: 997,
        test_998: 998,
        test_999: 999,
        test_1000: 1000,
    }
}
