use super::extended_euclidean_algorithm;

fn mod_inv(x: i32, n: i32) -> Option<i32> {
    let (g, x, _) = extended_euclidean_algorithm(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

pub fn chinese_remainder_theorem(residues: &[i32], modulli: &[i32]) -> Option<i32> {
    let prod = modulli.iter().product::<i32>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulli) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(chinese_remainder_theorem(&[3, 5, 7], &[2, 3, 1]), Some(5));
        assert_eq!(chinese_remainder_theorem(&[1, 4, 6], &[3, 5, 7]), Some(34));
        assert_eq!(chinese_remainder_theorem(&[1, 4, 6], &[1, 2, 0]), None);
        assert_eq!(chinese_remainder_theorem(&[2, 5, 7], &[6, 9, 15]), None);
    }
}
