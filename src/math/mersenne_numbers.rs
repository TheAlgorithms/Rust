// mersenne prime : https://en.wikipedia.org/wiki/Mersenne_prime
pub fn is_mersenne_number(n: usize) -> bool {
    if n == 2 {
        return true;
    }
    let mut s = 4;
    let m = 2_usize.pow(std::convert::TryInto::try_into(n).unwrap()) - 1;
    for _ in 0..n - 2 {
        s = ((s * s) - 2) % m;
    }
    s == 0
}

pub fn get_mersenne_numbers(limit: usize) -> Vec<usize> {
    let mut results: Vec<usize> = Vec::new();
    for num in 1..=limit {
        if is_mersenne_number(num) {
            results.push(num);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::{get_mersenne_numbers, is_mersenne_number};

    #[test]
    fn validity_check() {
        assert!(is_mersenne_number(3));
        assert!(is_mersenne_number(13));
        assert!(!is_mersenne_number(32));
    }

    #[allow(dead_code)]
    fn generation_check() {
        assert_eq!(get_mersenne_numbers(30), [2, 3, 5, 7, 13, 17, 19]);
    }
}
