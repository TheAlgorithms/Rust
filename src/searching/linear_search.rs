use std::cmp::PartialEq;

pub fn linear_search<T: PartialEq>(item: &T, arr: &[T]) -> Option<usize> {
    for (i, data) in arr.iter().enumerate() {
        if item == data {
            return Some(i);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    #[test]
    fn linear() {
        let index = super::linear_search(&"a", &vec!["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));

        let index = super::linear_search(&4, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = super::linear_search(&3, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = super::linear_search(&2, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = super::linear_search(&1, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(0));

        let index = super::linear_search(&5, &vec![1, 2, 3, 4]);
        assert_eq!(index, None);
    }
}
