// Author : cyrixninja
// Perfect Square : Checks if a number is perfect square number or not
// https://en.wikipedia.org/wiki/Perfect_square
pub fn perfect_square(num: i32) -> bool {
    if num < 0 {
        return false;
    }
    let sqrt_num = (num as f64).sqrt() as i32;
    sqrt_num * sqrt_num == num
}

pub fn perfect_square_binary_search(n: i32) -> bool {
    if n < 0 {
        return false;
    }

    let mut left = 0;
    let mut right = n;

    while left <= right {
        let mid = (left + right) / 2;
        let mid_squared = mid * mid;

        match mid_squared.cmp(&n) {
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Greater => right = mid - 1,
            std::cmp::Ordering::Less => left = mid + 1,
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_square() {
        assert!(perfect_square(9));
        assert!(perfect_square(81));
        assert!(perfect_square(4));
        assert!(perfect_square(0));
        assert!(!perfect_square(3));
        assert!(!perfect_square(-19));
    }

    #[test]
    fn test_perfect_square_binary_search() {
        assert!(perfect_square_binary_search(9));
        assert!(perfect_square_binary_search(81));
        assert!(perfect_square_binary_search(4));
        assert!(perfect_square_binary_search(0));
        assert!(!perfect_square_binary_search(3));
        assert!(!perfect_square_binary_search(-19));
    }
}
