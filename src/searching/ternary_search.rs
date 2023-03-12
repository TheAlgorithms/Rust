use std::cmp::Ordering;

pub fn ternary_search<T: Ord>(
    target: &T,
    list: &[T],
    mut start: usize,
    mut end: usize,
) -> Option<usize> {
    if list.is_empty() {
        return None;
    }

    while start <= end {
        let mid1: usize = start + (end - start) / 3;
        let mid2: usize = end - (end - start) / 3;

        match target.cmp(&list[mid1]) {
            Ordering::Less => end = mid1 - 1,
            Ordering::Equal => return Some(mid1),
            Ordering::Greater => match target.cmp(&list[mid2]) {
                Ordering::Greater => start = mid2 + 1,
                Ordering::Equal => return Some(mid2),
                Ordering::Less => {
                    start = mid1 + 1;
                    end = mid2 - 1;
                }
            },
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_none_if_empty_list() {
        let index = ternary_search(&"a", &[], 1, 10);
        assert_eq!(index, None);
    }

    #[test]
    fn returns_none_if_range_is_invalid() {
        let index = ternary_search(&1, &[1, 2, 3], 2, 1);
        assert_eq!(index, None);
    }

    #[test]
    fn returns_index_if_list_has_one_item() {
        let index = ternary_search(&1, &[1], 0, 1);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn returns_first_index() {
        let index = ternary_search(&1, &[1, 2, 3], 0, 2);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn returns_first_index_if_end_out_of_bounds() {
        let index = ternary_search(&1, &[1, 2, 3], 0, 3);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn returns_last_index() {
        let index = ternary_search(&3, &[1, 2, 3], 0, 2);
        assert_eq!(index, Some(2));
    }

    #[test]
    fn returns_last_index_if_end_out_of_bounds() {
        let index = ternary_search(&3, &[1, 2, 3], 0, 3);
        assert_eq!(index, Some(2));
    }

    #[test]
    fn returns_middle_index() {
        let index = ternary_search(&2, &[1, 2, 3], 0, 2);
        assert_eq!(index, Some(1));
    }

    #[test]
    fn returns_middle_index_if_end_out_of_bounds() {
        let index = ternary_search(&2, &[1, 2, 3], 0, 3);
        assert_eq!(index, Some(1));
    }
}
