use std::cmp::PartialEq;

pub fn linear_search<T: PartialEq>(item: &T, arr: &[T]) -> Option<usize> {
    for (i, data) in arr.iter().enumerate() {
        if item == data {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_strings() {
        let index = linear_search(&"a", &["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints() {
        let index = linear_search(&4, &[1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = linear_search(&3, &[1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = linear_search(&2, &[1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = linear_search(&1, &[1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = linear_search(&5, &[1, 2, 3, 4]);
        assert_eq!(index, None);
    }

    #[test]
    fn empty() {
        let index = linear_search(&1, &[]);
        assert_eq!(index, None);
    }
}
