/*
Linear Sieve algorithm:
Time complexity is indeed O(n) with O(n) memory, but the sieve generally
runs slower than a well implemented sieve of Eratosthenes. Some use cases are:
- factorizing any number k in the sieve in O(log(k))
- calculating arbitrary multiplicative functions on sieve numbers
  without increasing the time complexity
- As a by product, all prime numbers less than `max_number` are stored
  in `primes` vector.
 */
pub struct LinearSieve {
    max_number: usize,
    pub primes: Vec<usize>,
    pub minimum_prime_factor: Vec<usize>,
}

impl LinearSieve {
    pub const fn new() -> Self {
        LinearSieve {
            max_number: 0,
            primes: vec![],
            minimum_prime_factor: vec![],
        }
    }

    pub fn prepare(&mut self, max_number: usize) -> Result<(), &'static str> {
        if max_number <= 1 {
            return Err("Sieve size should be more than 1");
        }
        if self.max_number > 0 {
            return Err("Sieve already initialized");
        }
        self.max_number = max_number;
        self.minimum_prime_factor.resize(max_number + 1, 0);
        for i in 2..=max_number {
            if self.minimum_prime_factor[i] == 0 {
                self.minimum_prime_factor[i] = i;
                self.primes.push(i);
                /*
                   if needed, a multiplicative function can be
                   calculated for this prime number here:
                   function[i] = base_case(i);
                */
            }
            for p in self.primes.iter() {
                let mlt = (*p) * i;
                if *p > i || mlt > max_number {
                    break;
                }
                self.minimum_prime_factor[mlt] = *p;
                /*
                   multiplicative function for mlt can be calculated here:
                   if i % p:
                       function[mlt] = add_to_prime_exponent(function[i], i, p);
                   else:
                       function[mlt] = function[i] * function[p]
                */
            }
        }
        Ok(())
    }

    pub fn factorize(&self, mut number: usize) -> Result<Vec<usize>, &'static str> {
        if number > self.max_number {
            return Err("Number is too big, its minimum_prime_factor was not calculated");
        }
        if number == 0 {
            return Err("Number is zero");
        }
        let mut result: Vec<usize> = Vec::new();
        while number > 1 {
            result.push(self.minimum_prime_factor[number]);
            number /= self.minimum_prime_factor[number];
        }
        Ok(result)
    }
}

impl Default for LinearSieve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::LinearSieve;

    #[test]
    fn small_primes_list() {
        let mut ls = LinearSieve::new();
        ls.prepare(25).unwrap();
        assert_eq!(ls.primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
    }

    #[test]
    fn divisible_by_mpf() {
        let mut ls = LinearSieve::new();
        ls.prepare(1000).unwrap();
        for i in 2..=1000 {
            let div = i / ls.minimum_prime_factor[i];
            assert_eq!(i % ls.minimum_prime_factor[i], 0);
            if div == 1 {
                // Number must be prime
                assert!(ls.primes.binary_search(&i).is_ok());
            }
        }
    }

    #[test]
    fn check_factorization() {
        let mut ls = LinearSieve::new();
        ls.prepare(1000).unwrap();
        for i in 1..=1000 {
            let factorization = ls.factorize(i).unwrap();
            let mut product = 1usize;
            for (idx, p) in factorization.iter().enumerate() {
                assert!(ls.primes.binary_search(p).is_ok());
                product *= *p;
                if idx > 0 {
                    assert!(*p >= factorization[idx - 1]);
                }
            }
            assert_eq!(product, i);
        }
    }

    #[test]
    fn check_number_of_primes() {
        let mut ls = LinearSieve::new();
        ls.prepare(100_000).unwrap();
        assert_eq!(ls.primes.len(), 9592);
    }
}
