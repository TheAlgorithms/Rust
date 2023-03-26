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
        assert!(crate::sorting::is_sorted(&res));
    }

    #[test]
    fn already_sorted() {
        let res = pancake_sort(&mut ["a", "b", "c"]);
        assert!(crate::sorting::is_sorted(&res));
    }

    #[test]
    fn odd_number_of_elements() {
        let res = pancake_sort(&mut ["d", "a", "c", "e", "b"]);
        assert!(crate::sorting::is_sorted(&res));
    }

    #[test]
    fn one_element() {
        let res = pancake_sort(&mut [3]);
        assert!(crate::sorting::is_sorted(&res));
    }

    #[test]
    fn empty() {
        let res = pancake_sort(&mut [] as &mut [u8]);
        assert!(crate::sorting::is_sorted(&res));
    }
}
