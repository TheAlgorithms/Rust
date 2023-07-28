// Kth Factor of N
// The idea is to check for each number in the range [N, 1], and print the Kth number that divides N completely.

pub fn kth_factor(n: i32, k: i32) -> i32 {
    let mut factors: Vec<i32> = Vec::new();
    let k = (k as usize) - 1;
    for i in 1..=n {
        if n % i == 0 {
            factors.push(i);
        }
        if let Some(number) = factors.get(k) {
            return *number;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(kth_factor(12, 3), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(kth_factor(7, 2), 7);
    }

    #[test]
    fn test_3() {
        assert_eq!(kth_factor(4, 4), -1);
    }

    #[test]
    fn test_4() {
        assert_eq!(kth_factor(950, 5), 19);
    }
}
