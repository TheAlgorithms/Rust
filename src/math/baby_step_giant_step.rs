use crate::math::greatest_common_divisor;
/// Baby-step Giant-step algorithm
///
/// Solving discrete logarithm problem:
///     a^x = b (mod n) , with respect to gcd(a, n) == 1
/// with O(sqrt(n)) time complexity.
///
/// Wikipedia reference: https://en.wikipedia.org/wiki/Baby-step_giant-step
/// When a is the primitive root modulo n, the answer is unique.
/// Otherwise it will return the smallest positive solution
use std::collections::HashMap;

pub fn baby_step_giant_step(a: usize, b: usize, n: usize) -> Option<usize> {
    if greatest_common_divisor::greatest_common_divisor_stein(a as u64, n as u64) != 1 {
        return None;
    }

    let mut h_map = HashMap::new();
    let m = (n as f64).sqrt().ceil() as usize;
    // baby step
    let mut step = 1;
    for i in 0..m {
        h_map.insert((step * b) % n, i);
        step = (step * a) % n;
    }
    // Now step = a^m (mod n), giant step
    let giant_step = step;
    for i in (m..=n).step_by(m) {
        if let Some(v) = h_map.get(&step) {
            return Some(i - v);
        }
        step = (step * giant_step) % n;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::baby_step_giant_step;

    #[test]
    fn small_numbers() {
        assert_eq!(baby_step_giant_step(5, 3, 11), Some(2));
        assert_eq!(baby_step_giant_step(3, 83, 100), Some(9));
        assert_eq!(baby_step_giant_step(9, 1, 61), Some(5));
        assert_eq!(baby_step_giant_step(5, 1, 67), Some(22));
        assert_eq!(baby_step_giant_step(7, 1, 45), Some(12));
    }

    #[test]
    fn primitive_root_tests() {
        assert_eq!(
            baby_step_giant_step(3, 311401496, 998244353),
            Some(178105253)
        );
        assert_eq!(
            baby_step_giant_step(5, 324637211, 1000000007),
            Some(976653449)
        );
    }

    #[test]
    fn random_numbers() {
        assert_eq!(baby_step_giant_step(174857, 48604, 150991), Some(177));
        assert_eq!(baby_step_giant_step(912103, 53821, 75401), Some(2644));
        assert_eq!(baby_step_giant_step(448447, 365819, 671851), Some(23242));
        assert_eq!(
            baby_step_giant_step(220757103, 92430653, 434948279),
            Some(862704)
        );
        assert_eq!(
            baby_step_giant_step(176908456, 23538399, 142357679),
            Some(14215560)
        );
    }

    #[test]
    fn no_solution() {
        assert!(baby_step_giant_step(7, 6, 45).is_none());
        assert!(baby_step_giant_step(23, 15, 85).is_none());
        assert!(baby_step_giant_step(2, 1, 84).is_none());
    }
}
