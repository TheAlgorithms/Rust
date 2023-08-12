// Totient function for
// all numbers smaller than
// or equal to n.

// Computes and prints
// totient of all numbers
// smaller than or equal to n

use std::vec;

pub fn compute_totient(n: i32) -> vec::Vec<i32> {
    let mut phi: Vec<i32> = Vec::new();

    // initialize phi[i] = i
    for i in 0..=n {
        phi.push(i);
    }

    // Compute other Phi values
    for p in 2..n + 1 {
        // If phi[p] is not computed already,
        // then number p is prime
        if phi[(p) as usize] == p {
            // Phi of a prime number p is
            // always equal to p-1.
            phi[(p) as usize] = p - 1;

            // Update phi values of all
            // multiples of p
            for i in ((2 * p)..n + 1).step_by(p as usize) {
                phi[(i) as usize] = (phi[i as usize] / p) * (p - 1);
            }
        }
    }

    phi[1..].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            compute_totient(12),
            vec![1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(compute_totient(7), vec![1, 1, 2, 2, 4, 2, 6]);
    }

    #[test]
    fn test_3() {
        assert_eq!(compute_totient(4), vec![1, 1, 2, 2]);
    }
}
