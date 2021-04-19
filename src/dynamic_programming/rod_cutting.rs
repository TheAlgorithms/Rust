//! Compute the length of cuts that have the highest value

use std::cmp;

pub fn rod_cutting(price: &mut Vec<u32>) -> u32 {
    let length = price.len();

    if length <= 0 {
        return 0;
    }

    let mut val = vec![0; (length + 1) as usize];
    val[0] = 0;

    for j in 1..length + 1 {
        let mut max_val = 0;

        for i in 0..j {
            max_val = cmp::max(max_val, price[i] + val[j - i - 1]);
            val[j] = max_val;
        }
    }

    return val[length as usize];
}

pub fn rod_cutting_recursive(price: &mut Vec<u32>, length: u32) -> u32 {
    if length <= 0 {
        return 0;
    }

    let mut max_val = 0;

    for i in 0..length as usize {
        max_val = cmp::max(
            max_val,
            price[i] + rod_cutting_recursive(price, length - i as u32 - 1),
        );
    }

    return max_val;
}

#[cfg(test)]
mod test {
    use super::rod_cutting;
    use super::rod_cutting_recursive;

    #[test]
    fn test_rod_cutting() {
        assert_eq!(8, rod_cutting(&mut vec![1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(22, rod_cutting(&mut vec![1, 5, 8, 9, 10, 17, 17, 20]));
        assert_eq!(13, rod_cutting(&mut vec![1, 5, 8, 9, 10]));
    }

    #[test]
    fn test_rod_cutting_recursive() {
        assert_eq!(
            8,
            rod_cutting_recursive(&mut vec![1, 2, 3, 4, 5, 6, 7, 8], 8)
        );
        assert_eq!(
            22,
            rod_cutting_recursive(&mut vec![1, 5, 8, 9, 10, 17, 17, 20], 8)
        );
        assert_eq!(13, rod_cutting_recursive(&mut vec![1, 5, 8, 9, 10], 5));
    }
}
