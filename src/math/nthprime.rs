// Generate the nth prime number.
// Algorithm is inspired by the the optimized version of the Sieve of Eratosthenes.
pub fn nthprime(nth: u64) -> u64 {
    let mut total_prime: u64 = 0;
    let mut size_factor: u64 = 2;

    let mut s: u64 = nth * size_factor;
    let mut primes: Vec<u64> = Vec::new();

    let n: u64 = nth;

    while total_prime < n {
        primes = get_primes(s).to_vec();

        total_prime = primes[2..].iter().sum();
        size_factor += 1;
        s = n * size_factor;
    }

    count_prime(primes, n).unwrap()
}

fn get_primes(s: u64) -> Vec<u64> {
    let mut v: Vec<u64> = vec![1; s as usize];

    for index in 2..s {
        if v[index as usize] == 1 {
            for j in index..s {
                if index * j < s {
                    v[(index * j) as usize] = 0;
                } else {
                    break;
                }
            }
        }
    }
    v
}

fn count_prime(primes: Vec<u64>, n: u64) -> Option<u64> {
    let mut counter: u64 = 0;
    for i in 2..primes.len() {
        counter += primes.get(i).unwrap();
        if counter == n {
            return Some(i as u64);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn my_test() {
        assert_eq!(nthprime(100), 541u64);
    }
}
