pub fn merge_sort<T>(slice: &mut [T])
where
    T: Ord,
{
    merge_sort_cmp(slice, |a, b| a.lt(b));
}

pub fn merge_sort_cmp<T, F>(v: &mut [T], mut less: F)
where
    F: FnMut(&T, &T) -> bool,
{
    if std::mem::size_of::<T>() == 0 {
        return;
    }

    // Create a buffer with size `v.len() * size_of::<T>()` contains uninitialized data.
    let mut buf = Vec::with_capacity(v.len());

    sort_range(v, 0, v.len(), buf.as_mut_ptr(), &mut less);
}

fn sort_range<T, F>(v: &mut [T], begin: usize, end: usize, buf: *mut T, less: &mut F)
where
    F: FnMut(&T, &T) -> bool,
{
    if end - begin <= 1 {
        return;
    }

    let middle = (begin + end) / 2;

    sort_range(v, begin, middle, buf, less);
    sort_range(v, middle, end, buf, less);
    merge(v, begin, middle, end, buf, less);
}

fn merge<T, F>(v: &mut [T], begin: usize, middle: usize, end: usize, buf: *mut T, mut less: F)
where
    F: FnMut(&T, &T) -> bool,
{
    struct Range {
        begin: usize,
        end: usize,
    }

    let mut l = Range {
        begin: begin,
        end: middle,
    };
    let mut r = Range {
        begin: middle,
        end: end,
    };
    let mut top = unsafe { buf.add(begin) };

    unsafe fn push<T>(top: &mut *mut T, data: *const T) {
        std::ptr::copy_nonoverlapping(data, *top, 1);
        *top = top.add(1);
    }

    while l.begin < l.end && r.begin < r.end {
        if less(&v[l.begin], &v[r.begin]) {
            unsafe { push(&mut top, &v[l.begin]) };
            l.begin += 1;
        } else {
            unsafe { push(&mut top, &v[r.begin]) };
            r.begin += 1;
        }
    }

    for i in l.begin..l.end {
        unsafe { push(&mut top, &v[i]) };
    }
    for i in r.begin..r.end {
        unsafe { push(&mut top, &v[i]) };
    }

    unsafe {
        std::ptr::copy_nonoverlapping(buf.add(begin), v.as_mut_ptr().add(begin), end - begin)
    };
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
}
