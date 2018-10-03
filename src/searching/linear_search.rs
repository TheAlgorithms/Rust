use std::cmp::PartialEq;

pub fn linear_search<T: PartialEq>(item: &T, arr: &[T]) -> Option<usize> {
    for (i, data) in arr.iter().enumerate() {
        if item == data {
            return Some(i);
        }
    }

    return None;
}
