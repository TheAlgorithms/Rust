pub fn prime_check(num: usize) -> bool {
    if (num > 1) & (num < 4) {
        return true;
    } else if (num < 2) || (num % 2 == 0) {
        return false;
    }

    let stop: usize = (num as f64).sqrt() as usize + 1;
    for i in (3..stop).step_by(2) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert!(prime_check(3));
        assert!(prime_check(7));
        assert!(prime_check(11));
        assert!(prime_check(2003));

        assert!(!prime_check(4));
        assert!(!prime_check(6));
        assert!(!prime_check(21));
        assert!(!prime_check(2004));
    }
}
