pub fn sieve_of_eratosthenes(num: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    if num == 0 {
        return result;
    }
    let mut start: usize = 2;
    let end: usize = (num as f64).sqrt() as usize;
    let mut sieve: Vec<bool> = vec![true; num + 1];

    while start <= end {
        if sieve[start] {
            result.push(start);
            for i in (start * start..num + 1).step_by(start) {
                if sieve[i] {
                    sieve[i] = false;
                }
            }
        }
        start += 1;
    }
    for (i, item) in sieve.iter().enumerate().take(num + 1).skip(end + 1) {
        if *item {
            result.push(i)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(sieve_of_eratosthenes(0), vec![]);
        assert_eq!(sieve_of_eratosthenes(11), vec![2, 3, 5, 7, 11]);
        assert_eq!(
            sieve_of_eratosthenes(25),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23]
        );
        assert_eq!(
            sieve_of_eratosthenes(33),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]
        );
        assert_eq!(
            sieve_of_eratosthenes(100),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        );
    }
}
