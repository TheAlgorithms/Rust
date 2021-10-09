// https://projecteuler.net/problem=1
// Multiples of 3 or 5
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.
pub fn sum_of_multiples_of_3_or_5_below_1000() -> i32 {
    let mut sum = 0;
    for n in 1..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_multiples_of_3_or_5_below_1000() {
        assert_eq!(sum_of_multiples_of_3_or_5_below_1000(), 233168);
    }
}
