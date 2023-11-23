// Finds the prime factors of a number in increasing order, with repetition.

pub fn prime_factors(n: u64) -> Vec<u64> {
    let mut i = 2;
    let mut n = n;
    let mut factors = Vec::new();
    while i * i <= n {
        if n % i != 0 {
            if i != 2 {
                i += 1;
            }
            i += 1;
        } else {
            n /= i;
            factors.push(i);
        }
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(prime_factors(0), vec![]);
        assert_eq!(prime_factors(1), vec![]);
        assert_eq!(prime_factors(11), vec![11]);
        assert_eq!(prime_factors(25), vec![5, 5]);
        assert_eq!(prime_factors(33), vec![3, 11]);
        assert_eq!(prime_factors(2560), vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 5]);
    }
}
