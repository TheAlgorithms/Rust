use rand::seq::SliceRandom;
use rand::thread_rng;

fn is_sorted<T: Ord>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

pub fn bogo_bogo_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }

    let mut rng = thread_rng();
    let mut sorted_subarray = bogo_bogo_sort(&arr[..arr.len() - 1]);

    let mut extended_array = sorted_subarray.clone();
    extended_array.push(arr[arr.len() - 1].clone());

    while !is_sorted(&extended_array)
        || extended_array[arr.len() - 1] < *sorted_subarray.iter().max().unwrap()
    {
        extended_array.shuffle(&mut rng);
        sorted_subarray = bogo_bogo_sort(&extended_array[..arr.len() - 1]);
        extended_array = sorted_subarray.clone();
        extended_array.push(arr[arr.len() - 1].clone());
    }

    extended_array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_array() {
        let arr = vec![1, 2, 3, 4, 5];
        assert!(is_sorted(&bogo_bogo_sort(&arr)));
    }

    #[test]
    fn test_reverse_sorted_array() {
        let arr = vec![5, 4, 3, 2, 1];
        assert!(is_sorted(&bogo_bogo_sort(&arr)));
    }

    #[test]
    fn test_unsorted_array() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        assert!(is_sorted(&bogo_bogo_sort(&arr)));
    }

    #[test]
    fn test_empty_array() {
        let arr: Vec<i32> = vec![];
        assert!(is_sorted(&bogo_bogo_sort(&arr)));
    }
}
