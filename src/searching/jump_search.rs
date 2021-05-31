use std::cmp::min;

pub fn jump_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    //Base Case
    if arr.len() == 0 {
        return None;
    }

    let sqrt_arr: usize = (arr.len() as f64).sqrt() as usize;
    let mut step_size: usize = sqrt_arr;

    //Find the block that would contain the element
    let mut previous: usize = 0;
    while &arr[min(step_size, arr.len()) - 1] < item {
        previous = step_size;
        step_size += sqrt_arr;
        if previous >= arr.len() {
            return None;
        }
    }

    //Now we linear search that block
    while &arr[previous] < item {
        previous += 1;

        if previous == min(step_size, arr.len()) {
            return None;
        }
    }

    if &arr[previous] == item {
        return Some(previous);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = jump_search(&"a", &vec![]);
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
        let index = jump_search(&"a", &vec!["a"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings() {
        let index = jump_search(&"a", &vec!["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints() {
        let index = jump_search(&4, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = jump_search(&3, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = jump_search(&2, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = jump_search(&1, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = jump_search(&5, &vec![1, 2, 3, 4]);
        assert_eq!(index, None);
    }
}
