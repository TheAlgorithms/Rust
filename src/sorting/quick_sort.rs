extern crate std;

fn _display<T: std::fmt::Display>(arr: &[T]) {
    print!("\n");
    for item in arr {
        print!("{} ", item);
    }
}

fn _partition<T: Ord>(arr: &mut [T], lo: isize, hi: isize) -> isize {
    let pivot: *mut T = &mut arr[hi as usize];
    let mut i = lo - 1;
    let mut j = hi;

    unsafe {
        loop {
            i += 1;
            while arr[i as usize] < *pivot {
                i += 1;
            }
            j -= 1;
            while j >= 0 && arr[j as usize] > *pivot {
                j -= 1;
            }
            if i >= j {
                break;
            } else {
                arr.swap(i as usize, j as usize);
            }
        }
    }
    arr.swap(i as usize, hi as usize);
    return i;
}
fn _quick_sort<T: Ord>(arr: &mut [T], lo: isize, hi: isize) {
    if lo < hi {
        let p = _partition(arr, lo, hi);
        _quick_sort(arr, lo, p - 1);
        _quick_sort(arr, p + 1, hi);
    }
}
#[allow(dead_code)]
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    _quick_sort(arr, 0, (len - 1) as isize);
}
