use std::cmp::PartialOrd;

pub fn partition<T: PartialOrd>(arr: &mut [T], lo: isize, hi: isize) -> isize {
    let pivot = hi as usize;
    let mut i = lo - 1;
    let mut j = hi;

    loop {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && arr[j as usize] > arr[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap(i as usize, pivot as usize);
    i
}
fn _quick_sort<T: Ord>(arr: &mut [T], lo: isize, hi: isize) {
    if lo < hi {
        let p = partition(arr, lo, hi);
        _quick_sort(arr, lo, p - 1);
        _quick_sort(arr, p + 1, hi);
    }
}
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    _quick_sort(arr, 0, (len - 1) as isize);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut v = vec![6, 5, -8, 3, 2, 3];
        quick_sort(&mut v);
        assert_eq!(v, vec![-8, 2, 3, 3, 5, 6]);
    }

    #[test]
    fn already_sorted() {
        let mut v = vec!["a", "b", "c"];
        quick_sort(&mut v);
        assert_eq!(v, vec!["a", "b", "c"]);
    }

    #[test]
    fn odd_number_of_elements() {
        let mut v = vec!["d", "a", "c", "e", "b"];
        quick_sort(&mut v);
        assert_eq!(v, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn one_element() {
        let mut v = vec![3];
        quick_sort(&mut v);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn empty() {
        let mut v = Vec::<u8>::new();
        quick_sort(&mut v);
        assert_eq!(v, vec![]);
    }
}
