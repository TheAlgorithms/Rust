use std::cmp;

pub fn pancake_sort<T>(arr: &mut [T]) -> Vec<T>
where
    T: cmp::PartialEq + cmp::Ord + cmp::PartialOrd + Clone,
{
    let len = arr.len();
    if len < 2 {
        arr.to_vec();
    }
    for i in (0..len).rev() {
        let max_index = arr
            .iter()
            .take(i + 1)
            .enumerate()
            .max_by_key(|&(_, elem)| elem)
            .map(|(idx, _)| idx)
            .unwrap();
        if max_index != i {
            arr[0..max_index + 1].reverse();
            arr[0..i + 1].reverse();
        }
    }
    arr.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let res = pancake_sort(&mut [6, 5, -8, 3, 2, 3]);
        assert_eq!(res, vec![-8, 2, 3, 3, 5, 6]);
    }

    #[test]
    fn already_sorted() {
        let res = pancake_sort(&mut ["a", "b", "c"]);
        assert_eq!(res, vec!["a", "b", "c"]);
    }

    #[test]
    fn odd_number_of_elements() {
        let res = pancake_sort(&mut ["d", "a", "c", "e", "b"]);
        assert_eq!(res, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn one_element() {
        let res = pancake_sort(&mut [3]);
        assert_eq!(res, vec![3]);
    }

    #[test]
    fn empty() {
        let res = pancake_sort(&mut [] as &mut [u8]);
        assert_eq!(res, vec![]);
    }
}
