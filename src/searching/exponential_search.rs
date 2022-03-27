use std::cmp::Ordering;

pub fn exponential_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let len = arr.len();
    if len == 0 {
        return None;
    }
    let mut upper = 1;
    while (upper < len) && (&arr[upper] <= item) {
        upper *= 2;
    }
    if upper > len {
        upper = len
    }

    // binary search
    let mut lower = upper / 2;
    while lower < upper {
        let mid = lower + (upper - lower) / 2;

        match item.cmp(&arr[mid]) {
            Ordering::Less => upper = mid,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => lower = mid + 1,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = exponential_search(&"a", &vec![]);
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
        let index = exponential_search(&"a", &vec!["a"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings() {
        let index = exponential_search(&"a", &vec!["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints() {
        let index = exponential_search(&4, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = exponential_search(&3, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = exponential_search(&2, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = exponential_search(&1, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = exponential_search(&5, &vec![1, 2, 3, 4]);
        assert_eq!(index, None);
    }
}
