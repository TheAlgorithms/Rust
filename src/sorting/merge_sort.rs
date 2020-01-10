pub fn merge_sort<T: Ord + Copy + Default>(arr: &mut Vec<T>) {
    let len = arr.len();
    let mut tmp = vec![T::default(); len];
    m_sort(arr, &mut tmp, 0, len);
}

fn m_sort<T: Ord + Copy>(arr: &mut Vec<T>, tmp: &mut Vec<T>, left: usize, right: usize) {
    let mid = (right + left) / 2;
    if mid > left {
        m_sort(arr, tmp, left, mid);
        m_sort(arr, tmp, mid, right);

        merge(arr, tmp, left, right);
    }
}

fn merge<T: Ord + Copy>(arr: &mut Vec<T>, tmp: &mut Vec<T>, l: usize, r: usize) {
    let mut cpos = l;
    let mut left = l;
    let mut mid = (l + r) / 2;

    let lend = mid;

    while left < lend && mid < r {
        if arr[left] <= arr[mid] {
            tmp[cpos] = arr[left];
            left += 1;
        } else {
            tmp[cpos] = arr[mid];
            mid += 1;
        }
        cpos += 1;
    }

    while left < lend {
        tmp[cpos] = arr[left];
        left += 1;
        cpos += 1;
    }

    while mid < r {
        tmp[cpos] = arr[mid];
        mid += 1;
        cpos += 1;
    }

    for i in l..r {
        arr[i] = tmp[i];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let mut arr = vec![6, 5, 3, 1, 8, 7, 2, 4];
        merge_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    #[test]
    fn empty() {
        let mut arr: Vec<i32> = vec![];
        merge_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn reverse() {
        let mut arr = vec![8, 7, 6, 5, 4, 3, 2, 1];
        merge_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    #[test]
    fn already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        merge_sort(&mut arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}
