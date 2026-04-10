/*

    Moore's voting algorithm finds out the strictly majority-occurring element
    without using extra space
    and O(n) + O(n) time complexity

    It is built on the intuition that a strictly major element will always have a net occurrence as 1.
    Say, array given: 9 1 8 1 1
    Here, the algorithm will work as:

    (for finding element present >(n/2) times)
    (assumed: all elements are >0)

    Initialisation: ele=0, cnt=0
    Loop begins.

    loop 1: arr[0]=9
    ele = 9
    cnt=1 (since cnt = 0, cnt increments to 1 and ele = 9)

    loop 2: arr[1]=1
    ele = 9
    cnt= 0 (since in this turn of the loop, the array[i] != ele, cnt decrements by 1)

    loop 3: arr[2]=8
    ele = 8
    cnt=1 (since cnt = 0, cnt increments to 1 and ele = 8)

    loop 4: arr[3]=1
    ele = 8
    cnt= 0 (since in this turn of the loop, the array[i] != ele, cnt decrements by 1)

    loop 5: arr[4]=1
    ele = 9
    cnt=1 (since cnt = 0, cnt increments to 1 and ele = 1)

    Now, this ele should be the majority element if there's any
    To check, a quick O(n) loop is run to check if the count of ele is >(n/2), n being the length of the array

    -1 is returned when no such element is found.

*/

// boilerplate, because `==` isn't `const` yet
const fn eq_s(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

pub fn moore_voting_2pass<T: Eq>(arr: &[T]) -> Option<&T> {
    let mut ele = arr.first()?;
    let mut cnt = 0;

    for item in arr.iter() {
        if cnt == 0 {
            cnt = 1;
            ele = item;
        } else if item == ele {
            cnt += 1;
        } else {
            cnt -= 1;
        }
    }

    let cnt_check = arr.iter().filter(|&x| x == ele).count();

    let n = arr.len();
    if cnt_check > (n / 2) {
        Some(ele)
    } else {
        None
    }
}

pub const fn moore_voting_2pass_c<'a>(arr: &[&'a [u8]]) -> Option<&'a [u8]> {
    let n = arr.len();
    if n == 0 {
        return None;
    }
    let mut cnt: usize = 1;
    let mut ele = arr[0];
    let mut i = 1;
    while i < n {
        if cnt == 0 {
            cnt = 1;
            ele = arr[i];
        } else if eq_s(arr[i], ele) {
            cnt += 1;
        } else {
            cnt -= 1;
        }
        i += 1;
    }

    let mut cnt_check = 0;
    let mut i = 0;
    while i < n {
        if eq_s(arr[i], ele) {
            cnt_check += 1;
        }
        i += 1;
    }

    if cnt_check > (n / 2) {
        Some(ele)
    } else {
        None
    }
}

/// Returns `None` only if `i` is empty.
/// If there are multiple majorities, anyone could be returned.
///
/// # Panics
/// In debug-mode, if the internal majority-counter overflows.
/// The counter is `usize`, so it'll **never** overlow if `i` is a slice.
///
/// Even if `i` is infinite, the counter might never overflow;
/// consider this:
/// ```
///    core::iter::successors(Some(false), |b| Some(!b));
/// ```
/// This is equivalent to the sequence `1-1+1-1...`
pub fn moore_voting_it<T: Eq, I: IntoIterator<Item = T>>(it: I) -> Option<T> {
    let mut it = it.into_iter();
    let first = it.next()?;
    Some(
        it.fold((1_usize, first), |(cnt, ele), item| {
            if cnt == 0 {
                (1, item)
            } else if item == ele {
                (cnt + 1, ele)
            } else {
                (cnt - 1, ele)
            }
        })
        .1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moore_voting() {
        let arr1: Vec<i32> = vec![9, 1, 8, 1, 1];
        assert_eq!(moore_voting_2pass(&arr1), Some(&1));
        assert_eq!(moore_voting_it(arr1), Some(1));
        let arr2: Vec<i32> = vec![1, 2, 3, 4];
        assert_eq!(moore_voting_2pass(&arr2), None);
    }
}
