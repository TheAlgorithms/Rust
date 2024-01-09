pub fn combinations(n: i64, k: i64) -> i64 {
    if n < 0 || k < 0 {
        panic!("Please insert positive values");
    }
    let mut res: i64 = 1;
    for i in 0..k {
        res *= n - i;
        res /= i + 1;
    }
    res
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations_10_choose_5() {
        assert_eq!(combinations(10, 5), 252);
    }

    #[test]
    fn test_combinations_6_choose_3() {
        assert_eq!(combinations(6, 3), 20);
    }

    #[test]
    fn test_combinations_20_choose_5() {
        assert_eq!(combinations(20, 5), 15504);
    }

    #[test]
    #[should_panic(expected = "Please insert positive values")]
    fn test_combinations_invalid_input() {
        combinations(-5, 10);
    }
}