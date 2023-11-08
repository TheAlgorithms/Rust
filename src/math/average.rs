use num_traits::Num;
#[doc = r"# Average
Mean, Median, and Mode, in mathematics, the three principal ways of designating the average value of a list of numbers.
The arithmetic mean is found by adding the numbers and dividing the sum by the number of numbers in the list.
This is what is most often meant by an average. The median is the middle value in a list ordered from smallest to largest.
The mode is the most frequently occurring value on the list.

Reference: https://www.britannica.com/science/mean-median-and-mode

This program approximates the mean, median and mode of a finite sequence.
Note: `mean` function only limited to float 64 numbers. Floats sequences are not allowed for `median` & `mode` functions.
"]
use std::collections::HashMap;
use std::ops::{Add, Div, Sub};
/// # Argument
///
/// * `sequence` - A vector of float64 numbers.
/// Returns mean of `sequence`.
pub fn mean(sequence: Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;
    let n: f64 = sequence.len() as f64;
    for value in sequence {
        sum += value;
    }
    sum / n
}

fn mean_of_two<T: Num + Copy>(a: T, b: T) -> T {
    (a + b) / (T::one() + T::one())
}

/// # Argument
///
/// * `sequence` - A vector of numbers.
/// Returns median of `sequence`.

pub fn median<T: Num + Copy + PartialOrd>(mut sequence: Vec<T>) -> T {
    sequence.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if sequence.len() % 2 == 1 {
        let k = (sequence.len() + 1) / 2;
        sequence[k - 1]
    } else {
        let j = (sequence.len()) / 2;
        mean_of_two(sequence[j - 1], sequence[j])
    }
}

/// # Argument
///
/// * `sequence` - A vector of numbers.
/// Returns mode of `sequence`.
pub fn mode<
    T: Add<Output = T> + Sub<Output = T> + Div<i32, Output = T> + Ord + Copy + std::hash::Hash,
>(
    sequence: Vec<T>,
) -> T {
    let mut hash = HashMap::new();
    for value in sequence {
        let count = hash.entry(value).or_insert(0);
        *count += 1;
    }
    *hash.iter().max_by_key(|entry| entry.1).unwrap().0
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn median_test() {
        assert_eq!(median(vec![4, 53, 2, 1, 9, 0, 2, 3, 6]), 3);
        assert_eq!(median(vec![-9, -8, 0, 1, 2, 2, 3, 4, 6, 9, 53]), 2);
        assert_eq!(median(vec![2, 3]), 2);
        assert_eq!(median(vec![3.0, 2.0]), 2.5);
        assert_eq!(median(vec![1.0, 700.0, 5.0]), 5.0);
    }
    #[test]
    fn mode_test() {
        assert_eq!(mode(vec![4, 53, 2, 1, 9, 0, 2, 3, 6]), 2);
        assert_eq!(mode(vec![-9, -8, 0, 1, 2, 2, 3, -1, -1, 9, -1, -9]), -1);
    }
    #[test]
    fn mean_test() {
        assert_eq!(mean(vec![0.0, 1.0, 2.0, 3.0, 4.0]), 2.0);
        assert_eq!(
            mean(vec![-7.0, 4.0, 53.0, 2.0, 1.0, -9.0, 0.0, 2.0, 3.0, -6.0]),
            4.3
        );
    }
}
