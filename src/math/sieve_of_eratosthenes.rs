pub fn sieve_of_eratosthenes(n: usize) -> Vec<i32> {
    let mut primes: Vec<i32> = Vec::new();

    let mut is_prime = vec![true; n + 2];
    let limit: usize = ((n as f64).sqrt() as usize) + 2;
    for i in 2..limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n + 1 {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    for i in 2..(n + 1) {
        if is_prime[i] {
            primes.push(i as i32);
        }
    }
    primes
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sieve() {
        assert_eq!(sieve_of_eratosthenes(2).len(), 1);
        assert_eq!(sieve_of_eratosthenes(3).len(), 2);
        assert_eq!(sieve_of_eratosthenes(10).len(), 4);
    }
}
