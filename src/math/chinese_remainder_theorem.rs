use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    if a == 0 && b == 0 {
        return 0;
    }
    if b > a {
        gcd(b, a)
    } else {
        gcd(b, a % b)
    }
}
fn modinverse(x: u64, n: u64) -> Option<u64> {
    let g = gcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
#[allow(dead_code)]
fn xmodinverse(x: &BigInt, n: &BigInt) -> Option<BigInt> {
    let g = x.gcd(n);
    if g == BigInt::one() {
        Some((x % n + n) % n)
    } else {
        None
    }
}
pub fn chinese_remainder_theorem(residues: &[u64], modulli: &[u64]) -> Option<u64> {
    let prod = modulli.iter().product::<u64>();

    let mut sum = 0;

    for (residue, modulus) in residues.iter().zip(modulli) {
        let p = prod / modulus;
        let d = modinverse(p, *modulus)?;
        sum += residue * d * p;
    }
    Some(sum % prod)
}
#[allow(dead_code)]
pub fn xchinese_remainder_theorem(residues: &[BigInt], modulli: &[BigInt]) -> Option<BigInt> {
    let prod: BigInt = modulli.iter().product();

    let mut sum: BigInt = BigInt::zero();

    for (residue, modulus) in residues.iter().zip(modulli) {
        let p = prod.to_owned() / modulus;
        let d = xmodinverse(&p, modulus)?;
        sum += residue * d * p
    }
    Some(sum % prod)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        assert_eq!(chinese_remainder_theorem(&[3, 5, 7], &[2, 3, 1]), Some(5));
        assert_eq!(chinese_remainder_theorem(&[1, 4, 6], &[1, 2, 0]), None);
        assert_eq!(chinese_remainder_theorem(&[2, 5, 7], &[6, 9, 15]), None);
    }
    #[test]
    fn xbasic() {
        assert_eq!(
            xchinese_remainder_theorem(
                &[BigInt::from(3), BigInt::from(5), BigInt::from(7)],
                &[BigInt::from(2), BigInt::from(3), BigInt::from(1)]
            ),
            Some(BigInt::from(5))
        );
        assert_eq!(
            xchinese_remainder_theorem(
                &[BigInt::from(1), BigInt::from(4), BigInt::from(6)],
                &[BigInt::from(1), BigInt::from(2), BigInt::from(0)]
            ),
            None
        );
        assert_eq!(
            xchinese_remainder_theorem(
                &[BigInt::from(2), BigInt::from(5), BigInt::from(7)],
                &[BigInt::from(6), BigInt::from(9), BigInt::from(15)]
            ),
            None
        );
    }
}
