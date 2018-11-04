use std::cmp;

pub fn merge_sort<T>(arr: &[T]) -> Vec<T>
where
    T: cmp::PartialEq + cmp::PartialOrd + Clone,
{
    merge_sort_recur(arr)
}

pub fn merge_sort_recur<T>(arr: &[T]) -> Vec<T>
where
    T: cmp::PartialEq + cmp::PartialOrd + Clone,
{
    let l: usize = arr.len();

    if l <= 1 as usize {
        // already sorted
        return arr.to_vec();
    }

    let mut result: Vec<T> = Vec::with_capacity(arr.len());
    let mid: usize = l / 2;

    let l_sorted: Vec<T> = merge_sort_recur(&arr[..mid]);
    let r_sorted: Vec<T> = merge_sort_recur(&arr[mid..]);

    // merge step
    let (mut l_pointer, mut r_pointer) = (0, 0);

    while l_pointer < l_sorted.len() && r_pointer < r_sorted.len() {
        if l_sorted[l_pointer] < r_sorted[r_pointer] {
            result.push(l_sorted[l_pointer].clone());
            l_pointer += 1;
        } else {
            result.push(r_sorted[r_pointer].clone());
            r_pointer += 1;
        }
    }
    result.extend_from_slice(&l_sorted[l_pointer..]);
    result.extend_from_slice(&r_sorted[r_pointer..]);

    result.to_vec()
}

#[cfg(test)]
mod tests {

    #[test]
    fn merge_sort_empty() {
        let empty: [u32; 0] = [];
        let res = super::merge_sort(&empty);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn merge_sort_one() {
        let res = super::merge_sort(&[0]);
        assert_eq!(res, vec![0]);
    }

    #[test]
    fn merge_sort_sorted() {
        let res = super::merge_sort(&[1, 2, 3, 4, 5]);
        assert_eq!(res, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn merge_sort_unsorted() {
        let res = super::merge_sort(&[3, 2, 5, 4, 1, 6]);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn merge_sort_chars() {
        let res = super::merge_sort(&['a', 'x', 'e', 's', 'l']);
        assert_eq!(res, vec!['a', 'e', 'l', 's', 'x']);
    }
}
