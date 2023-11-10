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
        assert!(jump_search(&"a", &[]).is_none());
    }

    #[test]
    fn one_item() {
        assert_eq!(jump_search(&"a", &["a"]).unwrap(), 0);
    }

    #[test]
    fn search_strings() {
        assert_eq!(
            jump_search(&"a", &["a", "b", "c", "d", "google", "zoo"]).unwrap(),
            0
        );
    }

    #[test]
    fn search_ints() {
        let arr = [1, 2, 3, 4];
        assert_eq!(jump_search(&4, &arr).unwrap(), 3);
        assert_eq!(jump_search(&3, &arr).unwrap(), 2);
        assert_eq!(jump_search(&2, &arr).unwrap(), 1);
        assert_eq!(jump_search(&1, &arr).unwrap(), 0);
    }

    #[test]
    fn not_found() {
        let arr = [1, 2, 3, 4];

        assert!(jump_search(&5, &arr).is_none());
        assert!(jump_search(&0, &arr).is_none());
    }
}
