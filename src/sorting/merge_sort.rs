use std::cmp::Ordering;

fn _merge<T: Ord + Copy>(arr: &mut [T], lo: usize, mid: usize, hi: usize) {
    // create temporary arrays to support merge
    let mut left_half = Vec::new();
    let mut right_half = Vec::new();
    for i in lo..mid + 1 {
        left_half.push(arr[i]);
    }
    for i in mid + 1..hi + 1 {
        right_half.push(arr[i]);
    }

    let lsize = left_half.len();
    let rsize = right_half.len();

    // pointers to track the positions while merging
    let mut l = 0;
    let mut r = 0;
    let mut a = lo;

    // pick smaller element one by one from either left or right half
    while l < lsize && r < rsize {
        if left_half[l] < right_half[r] {
            arr[a] = left_half[l];
            l += 1;
        } else {
            arr[a] = right_half[r];
            r += 1;
        }
        a += 1;
    }

    // put all the remaining ones
    while l < lsize {
        arr[a] = left_half[l];
        l += 1;
        a += 1;
    }

    while r < rsize {
        arr[a] = right_half[r];
        r += 1;
        a += 1;
    }
}

fn _merge_sort<T: Ord + Copy>(arr: &mut [T], lo: usize, hi: usize) {
    if lo < hi {
        let mid = lo + (hi - lo) / 2;
        _merge_sort(arr, lo, mid);
        _merge_sort(arr, mid + 1, hi);
        _merge(arr, lo, mid, hi);
    }
}

pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len > 1 {
        _merge_sort(arr, 0, len - 1);
    }
}

pub fn merge_sort2<T: Ord>(mut v: Vec<T>) -> Vec<T> {
    if v.len() <= 1 {
        return v;
    }

    let mid = v.len() / 2;
    let u = v.split_off(mid);
    let mut u = merge_sort2(u).into_iter();
    let mut v = merge_sort2(v).into_iter();
    let mut res = Vec::new();
    let mut x = u.next();
    let mut y = v.next();
    loop {
        if x.is_some() && y.is_some() {
            match x.cmp(&y) {
                Ordering::Less => {
                    res.push(x.unwrap());
                    x = u.next();
                }
                _ => {
                    res.push(y.unwrap());
                    y = v.next();
                }
            }
        } else if x.is_some() {
            res.push(x.unwrap());
            x = u.next();
        } else if y.is_some() {
            res.push(y.unwrap());
            y = v.next();
        } else {
            break;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        merge_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        let res = merge_sort2(res);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        merge_sort(&mut res);
        assert_eq!(res, vec!["a", "bb", "cc", "d"]);

        let res = vec!["a", "bb", "d", "cc"];
        let res = merge_sort2(res);
        assert_eq!(res, vec!["a", "bb", "cc", "d"]);
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        merge_sort(&mut res);
        assert_eq!(res, vec![]);

        let res = Vec::<u8>::new();
        let res = merge_sort2(res);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        merge_sort(&mut res);
        assert_eq!(res, vec![1]);

        let res = vec![1];
        let res = merge_sort2(res);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        merge_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);

        let res = vec![1, 2, 3, 4];
        let res = merge_sort2(res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        merge_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);

        let res = vec![4, 3, 2, 1];
        let res = merge_sort2(res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }
}
