use std::cmp;

pub fn binary_search<T>(item: T, arr: &[T]) -> i32
where
    T: cmp::PartialEq + cmp::PartialOrd + Sized,
{
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;

        if arr[mid] > item {
            right = mid - 1;
        } else if arr[mid] < item {
            left = mid + 1;
        } else {
            left = mid;
            break;
        }
    }

    if arr[left] != item {
        return -1;
    }

    left as i32
}

pub fn linear_search<T>(item: T, arr: &[T]) -> i32
where
    T: cmp::PartialEq,
{
    let length = arr.len();

    for i in 0..length {
        if item == arr[i] {
            return i as i32;
        }
    }

    return -1;
}

#[cfg(test)]
mod tests {
    #[test]
    fn linear() {
        use searching;
        let index = searching::linear_search("a", &vec!["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, 0);

        let mut index = searching::linear_search(4, &vec![1, 2, 3, 4]);
        assert_eq!(index, 3);

        index = searching::linear_search(3, &vec![1, 2, 3, 4]);
        assert_eq!(index, 2);

        index = searching::linear_search(2, &vec![1, 2, 3, 4]);
        assert_eq!(index, 1);

        index = searching::linear_search(1, &vec![1, 2, 3, 4]);
        assert_eq!(index, 0);

        index = searching::linear_search(5, &vec![1, 2, 3, 4]);
        assert_eq!(index, -1);
    }
}
