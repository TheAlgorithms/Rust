#[doc = r"# Average
Mean, Median, and Mode, in mathematics, the three principal ways of designating the average value of a list of numbers.
The arithmetic mean is found by adding the numbers and dividing the sum by the number of numbers in the list.
This is what is most often meant by an average. The median is the middle value in a list ordered from smallest to largest.
The mode is the most frequently occurring value on the list.

Reference: https://www.britannica.com/science/mean-median-and-mode

This program approximates the mean, median and mode of a finite sequence.
Note: Floats sequences are not allowed for `mode` function.
"]
use std::collections::HashMap;
use std::collections::HashSet;

use num_traits::Num;

fn sum<T: Num + Copy>(sequence: Vec<T>) -> T {
    sequence.iter().fold(T::zero(), |acc, x| acc + *x)
}

/// # Argument
///
/// * `sequence` - A vector of numbers.
/// Returns mean of `sequence`.
pub fn mean<T: Num + Copy + num_traits::FromPrimitive>(sequence: Vec<T>) -> Option<T> {
    let len = sequence.len();
    if len == 0 {
        return None;
    }
    Some(sum(sequence) / (T::from_usize(len).unwrap()))
}

fn mean_of_two<T: Num + Copy>(a: T, b: T) -> T {
    (a + b) / (T::one() + T::one())
}

/// # Argument
///
/// * `sequence` - A vector of numbers.
/// Returns median of `sequence`.

pub fn median<T: Num + Copy + PartialOrd>(mut sequence: Vec<T>) -> Option<T> {
    if sequence.is_empty() {
        return None;
    }
    sequence.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if sequence.len() % 2 == 1 {
        let k = (sequence.len() + 1) / 2;
        Some(sequence[k - 1])
    } else {
        let j = (sequence.len()) / 2;
        Some(mean_of_two(sequence[j - 1], sequence[j]))
    }
}

fn histogram<T: Eq + std::hash::Hash>(sequence: Vec<T>) -> HashMap<T, usize> {
    sequence.into_iter().fold(HashMap::new(), |mut res, val| {
        *res.entry(val).or_insert(0) += 1;
        res
    })
}

/// # Argument
///
/// * `sequence` - The input vector.
/// Returns mode of `sequence`.
pub fn mode<T: Eq + std::hash::Hash>(sequence: Vec<T>) -> Option<HashSet<T>> {
    if sequence.is_empty() {
        return None;
    }
    let hist = histogram(sequence);
    let max_count = *hist.values().max().unwrap();
    Some(
        hist.into_iter()
            .filter(|(_, count)| *count == max_count)
            .map(|(value, _)| value)
            .collect(),
    )
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn median_test() {
        assert_eq!(median(vec![4, 53, 2, 1, 9, 0, 2, 3, 6]).unwrap(), 3);
        assert_eq!(median(vec![-9, -8, 0, 1, 2, 2, 3, 4, 6, 9, 53]).unwrap(), 2);
        assert_eq!(median(vec![2, 3]).unwrap(), 2);
        assert_eq!(median(vec![3.0, 2.0]).unwrap(), 2.5);
        assert_eq!(median(vec![1.0, 700.0, 5.0]).unwrap(), 5.0);
        assert!(median(Vec::<i32>::new()).is_none());
        assert!(median(Vec::<f64>::new()).is_none());
    }
    #[test]
    fn mode_test() {
        assert_eq!(
            mode(vec![4, 53, 2, 1, 9, 0, 2, 3, 6]).unwrap(),
            HashSet::from([2])
        );
        assert_eq!(
            mode(vec![-9, -8, 0, 1, 2, 2, 3, -1, -1, 9, -1, -9]).unwrap(),
            HashSet::from([-1])
        );
        assert_eq!(mode(vec!["a", "b", "a"]).unwrap(), HashSet::from(["a"]));
        assert_eq!(mode(vec![1, 2, 2, 1]).unwrap(), HashSet::from([1, 2]));
        assert_eq!(mode(vec![1, 2, 2, 1, 3]).unwrap(), HashSet::from([1, 2]));
        assert_eq!(mode(vec![1]).unwrap(), HashSet::from([1]));
        assert!(mode(Vec::<i32>::new()).is_none());
    }
    #[test]
    fn mean_test() {
        assert_eq!(mean(vec![2023.1112]).unwrap(), 2023.1112);
        assert_eq!(mean(vec![0.0, 1.0, 2.0, 3.0, 4.0]).unwrap(), 2.0);
        assert_eq!(
            mean(vec![-7.0, 4.0, 53.0, 2.0, 1.0, -9.0, 0.0, 2.0, 3.0, -6.0]).unwrap(),
            4.3
        );
        assert_eq!(mean(vec![1, 2]).unwrap(), 1);
        assert!(mean(Vec::<f64>::new()).is_none());
        assert!(mean(Vec::<i32>::new()).is_none());
    }
}
