use std::fmt::Display;
pub fn merge_sort<T: Ord + Copy + Display>(arr: &mut [T], tmp: &mut [T]) {
    let len = arr.len();
    m_sort(arr, tmp, 0, len);
}

fn m_sort<T: Ord + Copy + Display>(arr: &mut [T], tmp: &mut [T], left: usize, right: usize) {
    let mid = (right + left) / 2;
    if mid > left {
        m_sort(arr, tmp, left, mid);
        m_sort(arr, tmp, mid, right);

        merge(arr, tmp, left, right);
    }
    for i in 0 .. arr.len() {
        print!("{}, ", arr[i]);
    }
    println!("\n");
}

fn merge<T: Ord + Copy>(arr: &mut [T], tmp: &mut [T], l: usize, r: usize) {
    let mut cpos = l;
    let mut left = l;
    let mut mid = (l + r) / 2;

    let left_end = mid;
    println!("(left, mid, right): ({}, {}, {})", l, mid, r);

    while left < left_end && mid < r {
        if arr[left] <= arr[mid] {
            tmp[cpos] = arr[left];
            cpos+=1;
            left+=1;

        } else {
            tmp[cpos] = arr[mid];
            cpos+=1;
            mid+=1;
        }
    }

    while left < left_end {
        tmp[cpos] = arr[left];
        left+=1;
        cpos+=1;
    }

    while mid < r {
        tmp[cpos] = arr[mid];
        mid+=1;
        cpos+=1;
    }

    for i in l .. r {
        arr[i] = tmp[i];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let mut arr:[i32; 8] = [6, 5, 3, 1, 8, 7, 2, 4];
        let mut tmp: [i32; 8] = [0; 8];
        merge_sort(&mut arr, &mut tmp);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i+1]);
        }
    }

    #[test]
    fn empty() {
        let mut arr:[i32; 0] = [];
        let mut tmp: [i32; 0] = [];
        merge_sort(&mut arr, &mut tmp);
        assert_eq!(arr, []);
    }

    #[test]
    fn reverse() {
        let mut arr:[i32; 8] = [8, 7, 6, 5, 4, 3, 2, 1];
        let mut tmp: [i32; 8] = [0; 8];
        merge_sort(&mut arr, &mut tmp);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i+1]);
        }
    }

    #[test]
    fn already_sorted() {
        let mut arr:[i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let mut tmp: [i32; 8] = [0; 8];
        merge_sort(&mut arr, &mut tmp);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i+1]);
        }
    }
}
