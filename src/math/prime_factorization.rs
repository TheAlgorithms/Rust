use std::collections::BTreeMap;

use super::sieve_of_eratosthenes;

pub fn prime_factorization(n: i32) -> BTreeMap<i32, i32> {
    let mut m = n;
    let primes = sieve_of_eratosthenes(n as usize);
    let mut factorization: BTreeMap<i32, i32> = BTreeMap::new();
    for p in primes {
        if p * p > m {
            break;
        }
        while m % p == 0 {
            *factorization.entry(p).or_insert(0) += 1;
            m /= p;
        }
    }
    if m > 1 {
        factorization.insert(m, 1);
    }
    factorization
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_factorization() {
        let f1 = prime_factorization(10);
        assert_eq!(f1.get(&2), Some(&1), "{:?}", f1);
        assert_eq!(f1.get(&3), None, "{:?}", f1);
        assert_eq!(f1.get(&5), Some(&1), "{:?}", f1);
        let f2 = prime_factorization(24);
        assert_eq!(f2.get(&2), Some(&3), "{:?}", f2);
        assert_eq!(f2.get(&3), Some(&1), "{:?}", f2);
        assert_eq!(f2.get(&5), None, "{:?}", f2);
    }
}
