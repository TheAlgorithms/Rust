use std::cmp::Ordering;

pub fn ternary_search_rec<T: Ord>(
    target: &T,
    list: &[T],
    start: usize,
    end: usize,
) -> Option<usize> {
    if list.is_empty() {
        return None;
    }

    if end >= start {
        let mid1: usize = start + (end - start) / 3;
        let mid2: usize = end - (end - start) / 3;

        match target.cmp(&list[mid1]) {
            Ordering::Less => return ternary_search_rec(target, list, start, mid1 - 1),
            Ordering::Equal => return Some(mid1),
            Ordering::Greater => match target.cmp(&list[mid2]) {
                Ordering::Greater => return ternary_search_rec(target, list, mid2 + 1, end),
                Ordering::Equal => return Some(mid2),
                Ordering::Less => return ternary_search_rec(target, list, mid1 + 1, mid2 - 1),
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
        let index = ternary_search_rec(&"a", &[], 1, 10);
        assert_eq!(index, None);
    }

    #[test]
    fn returns_none_if_range_is_invalid() {
        let index = ternary_search_rec(&1, &[1, 2, 3], 2, 1);
        assert_eq!(index, None);
    }

    #[test]
    fn returns_index_if_list_has_one_item() {
        let index = ternary_search_rec(&1, &[1], 0, 1);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn returns_first_index() {
        let index = ternary_search_rec(&1, &[1, 2, 3], 0, 2);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn returns_first_index_if_end_out_of_bounds() {
        let index = ternary_search_rec(&1, &[1, 2, 3], 0, 3);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn returns_last_index() {
        let index = ternary_search_rec(&3, &[1, 2, 3], 0, 2);
        assert_eq!(index, Some(2));
    }

    #[test]
    fn returns_last_index_if_end_out_of_bounds() {
        let index = ternary_search_rec(&3, &[1, 2, 3], 0, 3);
        assert_eq!(index, Some(2));
    }

    #[test]
    fn returns_middle_index() {
        let index = ternary_search_rec(&2, &[1, 2, 3], 0, 2);
        assert_eq!(index, Some(1));
    }

    #[test]
    fn returns_middle_index_if_end_out_of_bounds() {
        let index = ternary_search_rec(&2, &[1, 2, 3], 0, 3);
        assert_eq!(index, Some(1));
    }
}
