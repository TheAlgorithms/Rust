/// Wave Sort Algorithm
///
/// Wave Sort is a sorting algorithm that works in O(n log n) time assuming
/// the sort function used works in O(n log n) time.
/// It arranges elements in an array into a sequence where every alternate
/// element is either greater or smaller than its adjacent elements.
///
/// Reference:
/// [Wave Sort Algorithm - GeeksforGeeks](https://www.geeksforgeeks.org/sort-array-wave-form-2/)
///
/// # Examples
///
/// use the_algorithms_rust::sorting::wave_sort;
/// let array = vec![10, 90, 49, 2, 1, 5, 23];
/// let result = wave_sort(array);
/// // Result: [2, 1, 10, 5, 49, 23, 90]
///
pub fn wave_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    arr.sort();

    for i in (0..n - 1).step_by(2) {
        arr.swap(i, i + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut array = vec![10, 90, 49, 2, 1, 5, 23];
        wave_sort(&mut array);
        let expected = vec![2, 1, 10, 5, 49, 23, 90];
        assert_eq!(&array, &expected);
    }

    #[test]
    fn test_case_2() {
        let mut array = vec![1, 3, 4, 2, 7, 8];
        wave_sort(&mut array);
        let expected = vec![2, 1, 4, 3, 8, 7];
        assert_eq!(&array, &expected);
    }

    #[test]
    fn test_case_3() {
        let mut array = vec![3, 3, 3, 3];
        wave_sort(&mut array);
        let expected = vec![3, 3, 3, 3];
        assert_eq!(&array, &expected);
    }

    #[test]
    fn test_case_4() {
        let mut array = vec![9, 4, 6, 8, 14, 3];
        wave_sort(&mut array);
        let expected = vec![4, 3, 8, 6, 14, 9];
        assert_eq!(&array, &expected);
    }

    #[test]
    fn test_case_5() {
        let mut array = vec![5, 10, 15, 20, 25];
        wave_sort(&mut array);
        let expected = vec![10, 5, 20, 15, 25];
        assert_eq!(&array, &expected);
    }
}
