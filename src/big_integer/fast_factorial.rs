// Algorithm created by Peter Borwein in 1985
// https://doi.org/10.1016/0196-6774(85)90006-9

use crate::math::sieve_of_eratosthenes;
use num_bigint::BigUint;
use num_traits::One;
use std::collections::BTreeMap;

/// Calculate the sum of n / p^i with integer division for all values of i
fn index(p: usize, n: usize) -> usize {
    let mut index = 0;
    let mut i = 1;
    let mut quot = n / p;

    while quot > 0 {
        index += quot;
        i += 1;
        quot = n / p.pow(i);
    }

    index
}

/// Calculate the factorial with time complexity O(log(log(n)) * M(n * log(n))) where M(n) is the time complexity of multiplying two n-digit numbers together.
pub fn fast_factorial(n: usize) -> BigUint {
    if n < 2 {
        return BigUint::one();
    }

    // get list of primes that will be factors of n!
    let primes = sieve_of_eratosthenes(n);

    let mut p_indeces = BTreeMap::new();

    // Map the primes with their index
    primes.into_iter().for_each(|p| {
        p_indeces.insert(p, index(p, n));
    });

    let max_bits = p_indeces.get(&2).unwrap().next_power_of_two().ilog2();

    // Create a Vec of 1's
    let mut a = Vec::with_capacity(max_bits as usize);
    a.resize(max_bits as usize, BigUint::one());

    // For every prime p, multiply a[i] by p if the ith bit of p's index is 1
    for (p, i) in p_indeces.into_iter() {
        let mut bit = 1usize;
        while bit.ilog2() < max_bits {
            if (bit & i) > 0 {
                a[bit.ilog2() as usize] *= p;
            }

            bit <<= 1;
        }
    }

    a.into_iter()
        .enumerate()
        .map(|(i, a_i)| a_i.pow(2u32.pow(i as u32))) // raise every a[i] to the 2^ith power
        .product() // we get our answer by multiplying the result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::big_integer::hello_bigmath::factorial;

    #[test]
    fn fact() {
        assert_eq!(fast_factorial(30), factorial(30));
        assert_eq!(fast_factorial(52), factorial(52));
        assert_eq!(fast_factorial(100), factorial(100));
        assert_eq!(fast_factorial(1000), factorial(1000));
        assert_eq!(fast_factorial(5000), factorial(5000));
    }
}
