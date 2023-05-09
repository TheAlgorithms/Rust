use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use std::cmp::Ordering;

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
    false
}

pub fn miller_rabin(number: u64, bases: &[u64]) -> u64 {
    // returns zero on a probable prime, and a witness if number is not prime
    // A base set for deterministic performance on 64 bit numbers is:
    // [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
    // another one for 32 bits:
    // [2, 3, 5, 7], with smallest number to fail 3'215'031'751 = 151 * 751 * 28351
    // note that all bases should be prime
    if number <= 4 {
        match number {
            0 => {
                panic!("0 is invalid input for Miller-Rabin. 0 is not prime by definition, but has no witness");
            }
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
    0
}

pub fn big_miller_rabin(number_ref: &BigUint, bases: &[u64]) -> u64 {
    let number = number_ref.clone();

    if BigUint::from(5u32).cmp(&number) == Ordering::Greater {
        if number.eq(&BigUint::zero()) {
            panic!("0 is invalid input for Miller-Rabin. 0 is not prime by definition, but has no witness");
        } else if number.eq(&BigUint::from(2u32)) || number.eq(&BigUint::from(3u32)) {
            return 0;
        } else {
            return number.to_u64().unwrap();
        }
    }

    if let Some(num) = number.to_u64() {
        if bases.contains(&num) {
            return 0;
        }
    }

    let num_minus_one = &number - BigUint::one();

    let two_power: u64 = num_minus_one.trailing_zeros().unwrap();
    let odd_power: BigUint = &num_minus_one >> two_power;
    for base in bases {
        let mut x = BigUint::from(*base).modpow(&odd_power, &number);

        if x.eq(&BigUint::one()) || x.eq(&num_minus_one) {
            continue;
        }

        let mut not_a_witness = false;

        for _ in 1..two_power {
            x = (&x * &x) % &number;
            if x.eq(&num_minus_one) {
                not_a_witness = true;
                break;
            }
        }

        if not_a_witness {
            continue;
        }

        return *base;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static DEFAULT_BASES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    #[test]
    fn basic() {
        // these bases make miller rabin deterministic for any number < 2 ^ 64
        // can use smaller number of bases for deterministic performance for numbers < 2 ^ 32

        assert_eq!(miller_rabin(3, &DEFAULT_BASES), 0);
        assert_eq!(miller_rabin(7, &DEFAULT_BASES), 0);
        assert_eq!(miller_rabin(11, &DEFAULT_BASES), 0);
        assert_eq!(miller_rabin(2003, &DEFAULT_BASES), 0);

        assert_ne!(miller_rabin(1, &DEFAULT_BASES), 0);
        assert_ne!(miller_rabin(4, &DEFAULT_BASES), 0);
        assert_ne!(miller_rabin(6, &DEFAULT_BASES), 0);
        assert_ne!(miller_rabin(21, &DEFAULT_BASES), 0);
        assert_ne!(miller_rabin(2004, &DEFAULT_BASES), 0);

        // bigger test cases.
        // primes are generated using openssl
        // non primes are randomly picked and checked using openssl

        // primes:
        assert_eq!(miller_rabin(3629611793, &DEFAULT_BASES), 0);
        assert_eq!(miller_rabin(871594686869, &DEFAULT_BASES), 0);
        assert_eq!(miller_rabin(968236663804121, &DEFAULT_BASES), 0);
        assert_eq!(miller_rabin(6920153791723773023, &DEFAULT_BASES), 0);

        // random non primes:
        assert_ne!(miller_rabin(4546167556336341257, &DEFAULT_BASES), 0);
        assert_ne!(miller_rabin(4363186415423517377, &DEFAULT_BASES), 0);
        assert_ne!(miller_rabin(815479701131020226, &DEFAULT_BASES), 0);
        // these two are made of two 31 bit prime factors:
        // 1950202127 * 2058609037 = 4014703722618821699
        assert_ne!(miller_rabin(4014703722618821699, &DEFAULT_BASES), 0);
        // 1679076769 * 2076341633 = 3486337000477823777
        assert_ne!(miller_rabin(3486337000477823777, &DEFAULT_BASES), 0);
    }

    #[test]
    fn big_basic() {
        assert_eq!(big_miller_rabin(&BigUint::from(3u32), &DEFAULT_BASES), 0);
        assert_eq!(big_miller_rabin(&BigUint::from(7u32), &DEFAULT_BASES), 0);
        assert_eq!(big_miller_rabin(&BigUint::from(11u32), &DEFAULT_BASES), 0);
        assert_eq!(big_miller_rabin(&BigUint::from(2003u32), &DEFAULT_BASES), 0);

        assert_ne!(big_miller_rabin(&BigUint::from(1u32), &DEFAULT_BASES), 0);
        assert_ne!(big_miller_rabin(&BigUint::from(4u32), &DEFAULT_BASES), 0);
        assert_ne!(big_miller_rabin(&BigUint::from(6u32), &DEFAULT_BASES), 0);
        assert_ne!(big_miller_rabin(&BigUint::from(21u32), &DEFAULT_BASES), 0);
        assert_ne!(big_miller_rabin(&BigUint::from(2004u32), &DEFAULT_BASES), 0);

        assert_eq!(
            big_miller_rabin(&BigUint::from(3629611793u64), &DEFAULT_BASES),
            0
        );
        assert_eq!(
            big_miller_rabin(&BigUint::from(871594686869u64), &DEFAULT_BASES),
            0
        );
        assert_eq!(
            big_miller_rabin(&BigUint::from(968236663804121u64), &DEFAULT_BASES),
            0
        );
        assert_eq!(
            big_miller_rabin(&BigUint::from(6920153791723773023u64), &DEFAULT_BASES),
            0
        );

        assert_ne!(
            big_miller_rabin(&BigUint::from(4546167556336341257u64), &DEFAULT_BASES),
            0
        );
        assert_ne!(
            big_miller_rabin(&BigUint::from(4363186415423517377u64), &DEFAULT_BASES),
            0
        );
        assert_ne!(
            big_miller_rabin(&BigUint::from(815479701131020226u64), &DEFAULT_BASES),
            0
        );
        assert_ne!(
            big_miller_rabin(&BigUint::from(4014703722618821699u64), &DEFAULT_BASES),
            0
        );
        assert_ne!(
            big_miller_rabin(&BigUint::from(3486337000477823777u64), &DEFAULT_BASES),
            0
        );
    }

    #[test]
    #[ignore]
    fn big_primes() {
        let p1 =
            BigUint::parse_bytes(b"4764862697132131451620315518348229845593592794669", 10).unwrap();
        assert_eq!(big_miller_rabin(&p1, &DEFAULT_BASES), 0);

        let p2 = BigUint::parse_bytes(
            b"12550757946601963214089118080443488976766669415957018428703",
            10,
        )
        .unwrap();
        assert_eq!(big_miller_rabin(&p2, &DEFAULT_BASES), 0);

        // An RSA-worthy prime
        let p3 = BigUint::parse_bytes(b"157d6l5zkv45ve4azfw7nyyjt6rzir2gcjoytjev5iacnkaii8hlkyk3op7bx9qfqiie23vj9iw4qbp7zupydfq9ut6mq6m36etya6cshtqi1yi9q5xyiws92el79dqt8qk7l2pqmxaa0sxhmd2vpaibo9dkfd029j1rvkwlw4724ctgaqs5jzy0bqi5pqdjc2xerhn", 36).unwrap();
        assert_eq!(big_miller_rabin(&p3, &DEFAULT_BASES), 0);

        let n1 = BigUint::parse_bytes(b"coy6tkiaqswmce1r03ycdif3t796wzjwneewbe3cmncaplm85jxzcpdmvy0moic3lql70a81t5qdn2apac0dndhohewkspuk1wyndxsgxs3ux4a7730unru7dfmygh", 36).unwrap();
        assert_ne!(big_miller_rabin(&n1, &DEFAULT_BASES), 0);

        // RSA-2048
        let n2 = BigUint::parse_bytes(b"4l91lq4a2sgekpv8ukx1gxsk7mfeks46haggorlkazm0oufxwijid6q6v44u5me3kz3ne6yczp4fcvo62oej72oe7pjjtyxgid5b8xdz1e8daafspbzcy1hd8i4urjh9hm0tyylsgqsss3jn372d6fmykpw4bb9cr1ngxnncsbod3kg49o7owzqnsci5pwqt8bch0t60gq0st2gyx7ii3mzhb1pp1yvjyor35hwvok1sxj3ih46rpd27li8y5yli3mgdttcn65k3szfa6rbcnbgkojqjjq72gar6raslnh6sjd2fy7yj3bwo43obvbg3ws8y28kpol3okb5b3fld03sq1kgrj2fugiaxgplva6x5ssilqq4g0b21xy2kiou3sqsgonmqx55v", 36).unwrap();
        assert_ne!(big_miller_rabin(&n2, &DEFAULT_BASES), 0);
    }
}
