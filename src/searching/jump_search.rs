use std::cmp::min;

pub fn jump_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let len = arr.len();
    if len == 0 {
        return None;
    }
    let mut step = (len as f64).sqrt() as usize;
    let mut prev = 0;

    while &arr[min(len, step) - 1] < item {
        prev = step;
        step += (len as f64).sqrt() as usize;
        if prev >= len {
            return None;
        }
    }
    while &arr[prev] < item {
        prev += 1;
        if prev == min(step, len) {
            return None;
        }
    }
    if &arr[prev] == item {
        return Some(prev);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = jump_search(&"a", &[]);
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
        let index = jump_search(&"a", &["a"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings() {
        let index = jump_search(&"a", &["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints() {
        let index = jump_search(&4, &[1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = jump_search(&3, &[1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = jump_search(&2, &[1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = jump_search(&1, &[1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = jump_search(&5, &[1, 2, 3, 4]);
        assert_eq!(index, None);

        assert!(jump_search(&0, &[1, 2, 3, 4]).is_none());
    }
}
