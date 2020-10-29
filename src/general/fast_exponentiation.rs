//Performs the fast exponentiation algorithm by repeated squaring operations
pub fn fast_exponentiation(base: u64, exponent: u64) -> u64 {
	if exponent == 0 {
		return 1;
	}

	if exponent == 1 {
		return base;
	}

	if exponent % 2 == 0 {
		return fast_exponentiation(base*base, exponent/2);
	} else {
		return base * fast_exponentiation(base*base, (exponent-1)/2);
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exp_zero() {
        assert_eq!(fast_exponentiation(10, 1), 10);
    }
    
    #[test]
    fn exp_one() {
        assert_eq!(fast_exponentiation(10, 1), 10);
    }

    #[test]
    fn exp_four() {
        assert_eq!(fast_exponentiation(10, 4), 10000);
    }

    #[test]
    fn exp_four_large() {
        assert_eq!(fast_exponentiation(4, 20), 4_u64.pow(20));
    }
}
