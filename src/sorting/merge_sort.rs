use std::fmt::Display;
pub fn merge_sort<T: Ord + Copy + Display>(arr: &mut [T], temp: &mut [T]) {
    let len = arr.len();
    m_sort(arr, temp, 0, len);
}

fn m_sort<T: Ord + Copy + Display>(arr: &mut [T], temp: &mut [T], left: usize, right: usize) {
    let mid = (right + left) / 2;
    if mid > left {
        m_sort(arr, temp, left, mid);
        m_sort(arr, temp, mid, right);

        merge(arr, temp, left, right);
    }
    for i in 0 .. arr.len() {
        print!("{}, ", arr[i]);
    }
    println!("\n");
}

fn merge<T: Ord + Copy>(arr: &mut [T], temp: &mut [T], l: usize, r: usize) {
    let mut cpos = l;
    let mut left = l;
    let mut mid = (l + r) / 2;

    let left_end = mid;
    println!("(left, mid, right): ({}, {}, {})", l, mid, r);

    while (left < left_end) && (mid < r) {
        if arr[left] <= arr[mid] {
            temp[cpos] = arr[left];
            cpos+=1;
            left+=1;
             
        } else {
            temp[cpos] = arr[mid];
            cpos+=1;
            mid+=1;
        }
    }

    while left < left_end {
        temp[cpos] = arr[left];
        left+=1;
        cpos+=1;
    }

    while mid < r {
        temp[cpos] = arr[mid];
        mid+=1;
        cpos+=1;
    }

    for right in (l .. r).rev() {
        arr[right] = temp[right];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let mut arr:[i32; 6] = [3, 5, 7, 6, 2, 1, 4];
        let mut temp: [i32; 6] = [0; arr.len()];
        merge_sort(&mut arr, &mut temp);
        for i in 0..arr.len() - 1 {
            println!("{}", arr[i]);
            assert!(arr[i] <= arr[i+1]);
        }
    }
}
