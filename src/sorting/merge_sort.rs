fn _merge<T: Ord + Copy>(arr: &mut [T], md: usize) {
    let len = arr.len();
    let mut merged = Vec::with_capacity(len);
    let lhs = &arr[..md];
    let rhs = &arr[md..];
    let mut i = 0 as usize;
    let mut j = 0 as usize;
    while i < lhs.len() || j < rhs.len() {
        if i < lhs.len() && (j == rhs.len() || lhs[i] < rhs[j]) {
            merged.push(lhs[i]);
            i += 1;
        } else {
            merged.push(rhs[j]);
            j += 1;
        }
    }
    arr.copy_from_slice(merged.as_slice());
}

fn _merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len > 1 {
        let md = len / 2;
        _merge_sort(&mut arr[..md]);
        _merge_sort(&mut arr[md..]);
        _merge(arr, md);
    }
}
pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    _merge_sort(arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut res = vec!["d", "a", "c", "b"];
        merge_sort(&mut res);
        assert_eq!(res, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        merge_sort(&mut res);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn one_element() {
        let mut res = vec!["a"];
        merge_sort(&mut res);
        assert_eq!(res, vec!["a"]);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec!["a", "b", "c"];
        merge_sort(&mut res);
        assert_eq!(res, vec!["a", "b", "c"]);
    }

    #[test]
    fn with_duplicated() {
        let mut res = vec!["d", "a", "c", "b", "c", "c", "c"];
        merge_sort(&mut res);
        assert_eq!(res, vec!["a", "b", "c", "c", "c", "c", "d"]);
    }
}
