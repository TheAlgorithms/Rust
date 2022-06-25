/*
Finds the product of two numbers using Karatsuba Algorithm
 */
use std::cmp::max;
const TEN: i128 = 10;

pub fn multiply(num1: i128, num2: i128) -> i128 {
    _multiply(num1, num2)
}

fn _multiply(num1: i128, num2: i128) -> i128 {
    if num1 < 10 || num2 < 10 {
        return num1 * num2;
    }
    let mut num1_str = num1.to_string();
    let mut num2_str = num2.to_string();

    let n = max(num1_str.len(), num2_str.len());
    num1_str = normalize(num1_str, n);
    num2_str = normalize(num2_str, n);

    let a = &num1_str[0..n / 2];
    let b = &num1_str[n / 2..];
    let c = &num2_str[0..n / 2];
    let d = &num2_str[n / 2..];

    let ac = _multiply(a.parse().unwrap(), c.parse().unwrap());
    let bd = _multiply(b.parse().unwrap(), d.parse().unwrap());
    let a_b: i128 = a.parse::<i128>().unwrap() + b.parse::<i128>().unwrap();
    let c_d: i128 = c.parse::<i128>().unwrap() + d.parse::<i128>().unwrap();
    let ad_bc = _multiply(a_b, c_d) - (ac + bd);

    let m = n / 2 + n % 2;
    (TEN.pow(2 * m as u32) * ac) + (TEN.pow(m as u32) * ad_bc) + (bd)
}

fn normalize(mut a: String, n: usize) -> String {
    for (counter, _) in (a.len()..n).enumerate() {
        a.insert(counter, '0');
    }
    a
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let n1: i128 = 314159265;
        let n2: i128 = 314159265;
        let ans = multiply(n1, n2);
        assert_eq!(ans, n1 * n2);
    }

    #[test]
    fn test_2() {
        let n1: i128 = 3141592653589793232;
        let n2: i128 = 2718281828459045233;
        let ans = multiply(n1, n2);
        assert_eq!(ans, n1 * n2);
    }

    #[test]
    fn test_3() {
        let n1: i128 = 123456789;
        let n2: i128 = 101112131415;
        let ans = multiply(n1, n2);
        assert_eq!(ans, n1 * n2);
    }
}
