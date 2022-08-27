use super::{mersenne_primes::is_mersenne_prime, prime_numbers::prime_numbers};
use std::convert::TryInto;

/*
    Generates a list of perfect numbers till `num` using the Lucas Lehmer test algorithm.
    url : https://en.wikipedia.org/wiki/Lucas%E2%80%93Lehmer_primality_test
*/
pub fn generate_perfect_numbers(num: usize) -> Vec<usize> {
    let mut results = Vec::new();
    let prime_limit = get_prime_limit(num);

    for i in prime_numbers(prime_limit).iter() {
        let prime = *i;
        if is_mersenne_prime(prime) {
            results.push(
                (2_usize.pow(prime.try_into().unwrap()) - 1)
                    * (2_usize.pow((prime - 1).try_into().unwrap())),
            );
        }
    }
    results.into_iter().filter(|x| *x <= num).collect()
}

// Gets an approximate limit for the generate_perfect_numbers function
fn get_prime_limit(num: usize) -> usize {
    (((num * 8 + 1) as f64).log2() as usize) / 2_usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_numbers_till_n() {
        let n = 335564540;
        assert_eq!(generate_perfect_numbers(n), [6, 28, 496, 8128, 33550336]);
        assert_eq!(generate_perfect_numbers(40), [6, 28]);
        assert_eq!(generate_perfect_numbers(0), []);
    }
}
