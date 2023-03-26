use std::cmp;

static MIN_MERGE: usize = 32;

fn min_run_length(mut n: usize) -> usize {
    let mut r = 0;
    while n >= MIN_MERGE {
        r |= n & 1;
        n >>= 1;
    }
    n + r
}

fn insertion_sort(arr: &mut Vec<i32>, left: usize, right: usize) -> &Vec<i32> {
    for i in (left + 1)..(right + 1) {
        let temp = arr[i];
        let mut j = (i - 1) as i32;

        while j >= (left as i32) && arr[j as usize] > temp {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = temp;
    }
    arr
}

fn merge(arr: &mut Vec<i32>, l: usize, m: usize, r: usize) -> &Vec<i32> {
    let len1 = m - l + 1;
    let len2 = r - m;
    let mut left = vec![0; len1];
    let mut right = vec![0; len2];

    left[..len1].clone_from_slice(&arr[l..(len1 + l)]);

    for x in 0..len2 {
        right[x] = arr[m + 1 + x];
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = l;

    while i < len1 && j < len2 {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < len1 {
        arr[k] = left[i];
        k += 1;
        i += 1;
    }

    while j < len2 {
        arr[k] = right[j];
        k += 1;
        j += 1;
    }
    arr
}

pub fn tim_sort(arr: &mut Vec<i32>, n: usize) {
    let min_run = min_run_length(MIN_MERGE);

    let mut i = 0;
    while i < n {
        insertion_sort(arr, i, cmp::min(i + MIN_MERGE - 1, n - 1));
        i += min_run;
    }

    let mut size = min_run;
    while size < n {
        let mut left = 0;
        while left < n {
            let mid = left + size - 1;
            let right = cmp::min(left + 2 * size - 1, n - 1);
            if mid < right {
                merge(arr, left, mid, right);
            }

            left += 2 * size;
        }
        size *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut array = vec![-2, 7, 15, -14, 0, 15, 0, 7, -7, -4, -13, 5, 8, -14, 12];
        let arr_len = array.len();
        tim_sort(&mut array, arr_len);
        assert!(crate::sorting::is_sorted(&array));
    }

    #[test]
    fn empty() {
        let mut array = Vec::<i32>::new();
        let arr_len = array.len();
        tim_sort(&mut array, arr_len);
        assert!(crate::sorting::is_sorted(&array));
    }

    #[test]
    fn one_element() {
        let mut array = vec![3];
        let arr_len = array.len();
        tim_sort(&mut array, arr_len);
        assert!(crate::sorting::is_sorted(&array));
    }

    #[test]
    fn pre_sorted() {
        let mut array = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let arr_len = array.len();
        tim_sort(&mut array, arr_len);
        assert!(crate::sorting::is_sorted(&array));
    }
}
