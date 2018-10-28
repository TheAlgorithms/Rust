pub fn merge_sort(numbers: &[u64]) -> Vec<u64> {
    let length = numbers.len();
    if length == 1 {
        return numbers.to_vec();
    }
    let middle = length / 2;

    let left_sorted = merge_sort(&numbers[..middle]);
    let right_sorted = merge_sort(&numbers[middle..]);

    _merge(left_sorted, right_sorted)
}

fn _merge(mut left: Vec<u64>, mut right: Vec<u64>) -> Vec<u64> {
    let total = left.len() + right.len();

    let mut sorted: Vec<u64> = vec![0; total];
    // reverse the sorted vectors to be able to pop the last element and compare
    left.reverse();
    right.reverse();

    for k in 0..total {
        if left.is_empty() {
            sorted[k] = right.pop().unwrap();
        } else if right.is_empty() {
            sorted[k] = left.pop().unwrap();
        } else if left.last() > right.last() {
            sorted[k] = right.pop().unwrap();
        } else {
            // else left < right
            sorted[k] = left.pop().unwrap();
        }
    }

    sorted
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn merge_sort_with_one_number() {
        assert_eq!(vec![12], merge_sort(&mut [12]));
    }

    #[test]
    pub fn merge_sort_with_sorted_numbers() {
        assert_eq!(
            vec![1, 12, 15, 19, 20, 21],
            merge_sort(&mut [1, 12, 15, 19, 20, 21])
        );
        assert_eq!(vec![1, 2, 3, 4, 5], merge_sort(&mut [1, 2, 3, 4, 5]));
    }

    #[test]
    pub fn merge_sort_with_unsorted_numbers() {
        assert_eq!(
            vec![1, 13, 15, 15, 50, 100],
            merge_sort(&mut [100, 15, 50, 1, 13, 15])
        );
        assert_eq!(vec![1, 2, 3, 4, 5], merge_sort(&mut [5, 4, 3, 2, 1]));
    }
}
