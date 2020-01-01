/// Sort a slice using "merge sort" algorithm. `T` must impl `Ord`.
///
/// If `T` has zero sized, nothing happened.
pub fn merge_sort<T>(slice: &mut [T])
where
    T: Ord,
{
    merge_sort_cmp(slice, |a, b| a.lt(b));
}

/// Sort a slice using "merge sort" algorithm. `less` is a `FnMut(&T, &T) -> bool`, `less(&lhs,
/// &rhs)` means called defined that `lhs` is less than `rhs`. The result order is undefined if
/// this `less` function not define a total ordering for elements in the slice.
///
/// If `T` has zero sized, nothing happened.
pub fn merge_sort_cmp<T, F>(v: &mut [T], mut less: F)
where
    F: FnMut(&T, &T) -> bool,
{
    if std::mem::size_of::<T>() == 0 {
        return;
    }

    // Create a buffer with size `v.len() * size_of::<T>()` contains uninitialized data.
    let mut buf = Vec::with_capacity(v.len());

    sort_range(v, buf.as_mut_ptr(), &mut less);
}

fn sort_range<T, F>(v: &mut [T], buf: *mut T, less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    if v.len() <= 1 {
        return;
    }

    let middle = v.len() / 2;
    {
        let (vl, vr) = v.split_at_mut(middle);
        sort_range(vl, buf, less);
        sort_range(vr, buf, less);
    }
    merge(v, middle, buf, less);
}

struct Range {
    begin: usize,
    end: usize,
}

unsafe fn push_top<T>(top: &mut *mut T, data: *const T) {
    std::ptr::copy_nonoverlapping(data, *top, 1);
    *top = top.add(1);
}

fn merge<T, F>(v: &mut [T], middle: usize, buf: *mut T, mut less: F)
where
    F: FnMut(&T, &T) -> bool,
{
    let mut l = Range {
        begin: 0,
        end: middle,
    };
    let mut r = Range {
        begin: middle,
        end: v.len(),
    };
    let mut top = buf;

    while l.begin < l.end && r.begin < r.end {
        if less(&v[l.begin], &v[r.begin]) {
            unsafe { push_top(&mut top, &v[l.begin]) };
            l.begin += 1;
        } else {
            unsafe { push_top(&mut top, &v[r.begin]) };
            r.begin += 1;
        }
    }

    for i in l.begin..l.end {
        unsafe { push_top(&mut top, &v[i]) };
    }
    for i in r.begin..r.end {
        unsafe { push_top(&mut top, &v[i]) };
    }

    unsafe { std::ptr::copy_nonoverlapping(buf, v.as_mut_ptr(), v.len()) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increase() {
        let mut v = vec![1, 2, 3, 4];
        let answer = v.clone();

        merge_sort(&mut v);

        assert_eq!(v, answer);
    }

    #[test]
    fn decrease() {
        let mut v = vec![4, 3, 2, 1];

        let mut answer = v.clone();
        answer.reverse();

        merge_sort(&mut v);
        assert_eq!(v, answer);
    }

    #[test]
    fn urandom() {
        let mut v = vec![];
        let (mut x, a, c, m) = (0u64, 12, 34, 1_000_000_000 + 7);

        for _ in 0..65536 {
            x = (x + a) * c % m;
            v.push(x);
        }

        let mut answer = v.clone();
        answer.sort();

        merge_sort(&mut v);
        assert_eq!(v, answer);
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Point {
        x: u32,
        y: u32,
    }

    #[test]
    fn no_clone() {
        let mut v = vec![];

        v.push(Point { x: 10, y: 9 });
        v.push(Point { x: 3, y: 8 });
        v.push(Point { x: 8, y: 7 });

        merge_sort_cmp(&mut v, |l, r| l.x < r.x);
        assert_eq!(v[0], Point { x: 3, y: 8 });
        assert_eq!(v[1], Point { x: 8, y: 7 });
        assert_eq!(v[2], Point { x: 10, y: 9 });

        merge_sort_cmp(&mut v, |l, r| l.y < r.y);
        assert_eq!(v[0], Point { x: 8, y: 7 });
        assert_eq!(v[1], Point { x: 3, y: 8 });
        assert_eq!(v[2], Point { x: 10, y: 9 });
    }
}
