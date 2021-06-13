fn _partition<T: Ord>(arr: &mut [T], lo: isize, hi: isize) -> isize {
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
        let p = _partition(arr, lo, hi);
        _quick_sort(arr, lo, p - 1);
        _quick_sort(arr, p + 1, hi);
    }
}
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    _quick_sort(arr, 0, (len - 1) as isize);
}


pub fn sort_in_three_way<T>(a: &mut [T]) 
where 
    T: Clone + PartialOrd 
{
    if a.len() == 0 {
        return;
    }
    sort_in_three_way_core(a, 0, a.len() - 1);
}
pub fn sort_in_three_way_core<T>(a: &mut [T], start: usize, end: usize)
where 
    T: Clone + PartialOrd
{
    if start >= end {
        return;
    }
    let (mut l, mut i, mut r) = (start, start, end);
    let pivot_val = a[start].clone();
    while i <= r {
        if a[i] == pivot_val {
            i += 1;
        } else if a[i] > pivot_val {
            a.swap(i, r);
            if r == 0 {
                break;
            }
            r -= 1;
        } else if a[i] < pivot_val {
            a.swap(i, l);
            l += 1;
            i += 1;
        }
    }
    if l > 0 {
        sort_three_way_range(a, start, l - 1);
    } 
    sort_three_way_range(a, r + 1, end);
}