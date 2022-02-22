fn modulo_power(mut base: u64, mut power: u64, modulo: u64) -> u64 {
    base %= modulo;
    if base == 0 {
        return 0; // return zero if base is divisible by modulo
    }
    let mut ans: u128 = 1;
    let mut bbase: u128 = base as u128;
    while power > 0 {
        if (power % 2) == 1 {
            ans = (ans * bbase) % (modulo as u128);
        }
        bbase = (bbase * bbase) % (modulo as u128);
        power /= 2;
    }
    ans as u64
}

fn check_prime_base(number: u64, base: u64, two_power: u64, odd_power: u64) -> bool {
    // returns false if base is a witness
    let mut x: u128 = modulo_power(base, odd_power, number) as u128;
    let bnumber: u128 = number as u128;
    if x == 1 || x == (bnumber - 1) {
        return true;
    }
    for _ in 1..two_power {
        x = (x * x) % bnumber;
        if x == (bnumber - 1) {
            return true;
        }
    }
    return false;
}

pub fn miller_rabin(number: u64, bases: &Vec<u64>) -> u64 {
    // returns zero on a probable prime, and a witness if number is not prime
    // A base set for deterministic performance on 64 bit numbers is:
    // [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
    // another one for 32 bits:
    // [2, 3, 5, 7], with smallest number to fail 3'215'031'751 = 151 * 751 * 28351
    // note that all bases should be prime
    if number <= 4 {
        match number {
            2 => return 0,
            3 => return 0,
            _ => return number,
        }
    }
    if bases.contains(&number) {
        return 0;
    }
    let two_power: u64 = (number - 1).trailing_zeros() as u64;
    let odd_power = (number - 1) >> two_power;
    for base in bases {
        if !check_prime_base(number, *base, two_power, odd_power) {
            return *base;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let default_bases: Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
        // these bases make miller rabin deterministic for any number < 2 ^ 64
        // can use smaller number of bases for deterministic performance for numbers < 2 ^ 32

        assert_eq!(miller_rabin(3, &default_bases), 0);
        assert_eq!(miller_rabin(7, &default_bases), 0);
        assert_eq!(miller_rabin(11, &default_bases), 0);
        assert_eq!(miller_rabin(2003, &default_bases), 0);

        assert_ne!(miller_rabin(1, &default_bases), 0);
        assert_ne!(miller_rabin(4, &default_bases), 0);
        assert_ne!(miller_rabin(6, &default_bases), 0);
        assert_ne!(miller_rabin(21, &default_bases), 0);
        assert_ne!(miller_rabin(2004, &default_bases), 0);

        // bigger test cases.
        // primes are generated using openssl
        // non primes are randomly picked and checked using openssl

        // primes:
        assert_eq!(miller_rabin(3629611793, &default_bases), 0);
        assert_eq!(miller_rabin(871594686869, &default_bases), 0);
        assert_eq!(miller_rabin(968236663804121, &default_bases), 0);
        assert_eq!(miller_rabin(6920153791723773023, &default_bases), 0);

        // random non primes:
        assert_ne!(miller_rabin(4546167556336341257, &default_bases), 0);
        assert_ne!(miller_rabin(4363186415423517377, &default_bases), 0);
        assert_ne!(miller_rabin(815479701131020226, &default_bases), 0);
        // these two are made of two 31 bit prime factors:
        // 1950202127 * 2058609037 = 4014703722618821699
        assert_ne!(miller_rabin(4014703722618821699, &default_bases), 0);
        // 1679076769 * 2076341633 = 3486337000477823777
        assert_ne!(miller_rabin(3486337000477823777, &default_bases), 0);
    }
}
