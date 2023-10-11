fn _binary_search<T: Ord>(arr: &[T], target: &T) -> usize {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;

        if arr[mid] < *target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    low
}

pub fn binary_insertion_sort<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();

    for i in 1..len {
        let key = arr[i].clone();
        let index = _binary_search(&arr[..i], &key);

        arr[index..i + 1].rotate_right(1);
        arr[index] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_insertion_sort() {
        let mut arr1 = vec![64, 25, 12, 22, 11];
        let mut arr2 = vec![5, 4, 3, 2, 1];
        let mut arr3 = vec![1, 2, 3, 4, 5];
        let mut arr4: Vec<i32> = vec![]; // Explicitly specify the type for arr4

        binary_insertion_sort(&mut arr1);
        binary_insertion_sort(&mut arr2);
        binary_insertion_sort(&mut arr3);
        binary_insertion_sort(&mut arr4);

        assert_eq!(arr1, vec![11, 12, 22, 25, 64]);
        assert_eq!(arr2, vec![1, 2, 3, 4, 5]);
        assert_eq!(arr3, vec![1, 2, 3, 4, 5]);
        assert_eq!(arr4, Vec::<i32>::new());
    }
}
