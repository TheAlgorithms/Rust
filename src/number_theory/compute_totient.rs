// Totient function for
// all numbers smaller than
// or equal to n.

// Computes and prints
// totient of all numbers
// smaller than or equal to n

use std::vec;

pub fn compute_totient(n: usize) -> Vec<usize> {
    if n == 0 {
        return vec![];
    }
    
    let mut phi: Vec<usize> = (0..=n).collect();
    
    for p in 2..=n {
        if phi[p] == p {
            phi[p] = p - 1;
            let step = p;
            for i in ((2 * p)..=n).step_by(step) {
                phi[i] = (phi[i] / p) * (p - 1);
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
    #[test]
    fn test_edge_cases() {
        assert_eq!(compute_totient(0), vec![]);
        assert_eq!(compute_totient(1), vec![1]);
        assert_eq!(compute_totient(2), vec![1, 1]);
    }

    #[test]
    fn test_prime_numbers() {
        // For prime p, φ(p) = p-1
        assert_eq!(compute_totient(13), vec![1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 2, 12]);
        assert_eq!(compute_totient(17).last(), Some(&16));
    }

    #[test]
    fn test_powers_of_two() {
        // For 2^k, φ(2^k) = 2^(k-1)
        assert_eq!(compute_totient(8), vec![1, 1, 2, 2, 4, 2, 6, 4]);
        // Verify φ(8) = 4
        assert_eq!(compute_totient(8)[7], 4);
    }

    #[test]
    fn test_small_values() {
        let result = compute_totient(10);
        // Known values: φ(1)=1, φ(2)=1, φ(3)=2, φ(4)=2, φ(5)=4, 
        // φ(6)=2, φ(7)=6, φ(8)=4, φ(9)=6, φ(10)=4
        assert_eq!(result, vec![1, 1, 2, 2, 4, 2, 6, 4, 6, 4]);
    }

    #[test]
    fn test_property_multiplicative() {
        // φ(mn) = φ(m) * φ(n) when m,n are coprime
        let result = compute_totient(15); // 15 = 3 * 5
        // φ(3)=2, φ(5)=4, so φ(15)=8
        assert_eq!(result[14], 8);
    }

    #[test]
    fn test_larger_n() {
        let result = compute_totient(20);
        assert_eq!(result.len(), 20);
        // Spot check known values
        assert_eq!(result[0], 1);  // φ(1)
        assert_eq!(result[6], 6);  // φ(7)
        assert_eq!(result[8], 6);  // φ(9)
        assert_eq!(result[11], 4); // φ(12)
        assert_eq!(result[19], 8); // φ(20)
    }

    #[test]
    fn test_consistency() {
        // Test that φ(n) ≤ n-1 for n > 1
        for n in 2..=50 {
            let result = compute_totient(n);
            assert!(result[n-1] <= n-1, "φ({}) = {} exceeds {}", n, result[n-1], n-1);
        }
    }

    #[test]
    fn test_sieve_correctness() {
        // Compare with direct computation for small n
        fn direct_phi(n: usize) -> usize {
            (1..=n).filter(|&x| gcd(x, n) == 1).count()
        }
        
        fn gcd(a: usize, b: usize) -> usize {
            if b == 0 { a } else { gcd(b, a % b) }
        }
        
        for n in 1..=20 {
            let sieve_result = compute_totient(n);
            let direct_result: Vec<usize> = (1..=n).map(direct_phi).collect();
            assert_eq!(sieve_result, direct_result, "Failed for n={}", n);
        }
    }
}
