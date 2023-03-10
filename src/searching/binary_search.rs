use std::cmp::Ordering;

pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let mut is_asc = true;
    if arr.len() > 1 {
        is_asc = arr[0] < arr[arr.len() - 1];
    }
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if is_asc {
            match item.cmp(&arr[mid]) {
                Ordering::Less => right = mid,
                Ordering::Equal => return Some(mid),
                Ordering::Greater => left = mid + 1,
            }
        } else {
            match item.cmp(&arr[mid]) {
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return Some(mid),
                Ordering::Greater => right = mid,
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = binary_search(&"a", &[]);
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
        let index = binary_search(&"a", &["a"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings_asc() {
        let index = binary_search(&"a", &["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));

        let index = binary_search(&"google", &["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(4));
    }

    #[test]
    fn search_strings_desc() {
        let index = binary_search(&"a", &["zoo", "google", "d", "c", "b", "a"]);
        assert_eq!(index, Some(5));

        let index = binary_search(&"zoo", &["zoo", "google", "d", "c", "b", "a"]);
        assert_eq!(index, Some(0));

        let index = binary_search(&"google", &["zoo", "google", "d", "c", "b", "a"]);
        assert_eq!(index, Some(1));
    }

    #[test]
    fn search_ints_asc() {
        let index = binary_search(&4, &[1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = binary_search(&3, &[1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = binary_search(&2, &[1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = binary_search(&1, &[1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints_desc() {
        let index = binary_search(&4, &[4, 3, 2, 1]);
        assert_eq!(index, Some(0));

        let index = binary_search(&3, &[4, 3, 2, 1]);
        assert_eq!(index, Some(1));

        let index = binary_search(&2, &[4, 3, 2, 1]);
        assert_eq!(index, Some(2));

        let index = binary_search(&1, &[4, 3, 2, 1]);
        assert_eq!(index, Some(3));
    }

    #[test]
    fn not_found() {
        let index = binary_search(&5, &[1, 2, 3, 4]);
        assert_eq!(index, None);

        let index = binary_search(&5, &[4, 3, 2, 1]);
        assert_eq!(index, None);
    }
}
