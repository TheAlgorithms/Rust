use std::cmp::{Ord, Ordering};

fn _quick_sort_3_ways<T: Ord>(arr: &mut [T], lo: isize, hi: isize) {
    if lo >= hi {
        return;
    }

    let mut lt = lo; // arr[lo+1, lt] < v
    let mut gt = hi + 1; // arr[gt, r] > v
    let mut i = lo + 1; // arr[lt + 1, i) == v

    while i < gt {
        match arr[i as usize].cmp(&arr[lo as usize]) {
            Ordering::Less => {
                arr.swap(i as usize, (lt + 1) as usize);
                i += 1;
                lt += 1;
            }
            Ordering::Greater => {
                arr.swap(i as usize, (gt - 1) as usize);
                gt -= 1;
            }
            Ordering::Equal => {
                i += 1;
            }
        }
    }

    arr.swap(lo as usize, lt as usize);

    _quick_sort_3_ways(arr, lo, lt - 1);
    _quick_sort_3_ways(arr, gt, hi);
}

pub fn quick_sort_3_ways<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len > 1 {
        _quick_sort_3_ways(arr, 0, (len - 1) as isize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;
    use crate::sorting::sort_utils;

    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        let cloned = res.clone();
        sort_utils::log_timed("basic", || {
            quick_sort_3_ways(&mut res);
        });

        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        let cloned = res.clone();
        sort_utils::log_timed("basic sring", || {
            quick_sort_3_ways(&mut res);
        });

        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        let cloned = res.clone();
        sort_utils::log_timed("empty", || {
            quick_sort_3_ways(&mut res);
        });

        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn one_element() {
        let mut res = sort_utils::generate_random_vec(1, 0, 1);
        let cloned = res.clone();
        sort_utils::log_timed("one element", || {
            quick_sort_3_ways(&mut res);
        });

        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut res = sort_utils::generate_nearly_ordered_vec(1000, 0);
        let cloned = res.clone();
        sort_utils::log_timed("pre sorted", || {
            quick_sort_3_ways(&mut res);
        });

        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn reverse_sorted() {
        let mut res = sort_utils::generate_reverse_ordered_vec(5);
        let cloned = res.clone();
        sort_utils::log_timed("reverse sorted", || {
            quick_sort_3_ways(&mut res);
        });

        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn large_elements() {
        let mut res = sort_utils::generate_random_vec(1000000, 0, 1000000);
        let cloned = res.clone();
        sort_utils::log_timed("large elements test", || {
            quick_sort_3_ways(&mut res);
        });
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    #[ignore = "too longer time to run"]
    fn nearly_ordered_elements() {
        let mut res = sort_utils::generate_nearly_ordered_vec(1000000, 10);
        let cloned = res.clone();

        sort_utils::log_timed("nearly ordered elements test", || {
            quick_sort_3_ways(&mut res);
        });

        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn repeated_elements() {
        let mut res = sort_utils::generate_repeated_elements_vec(1000000, 3);
        let cloned = res.clone();

        sort_utils::log_timed("repeated elements test", || {
            quick_sort_3_ways(&mut res);
        });

        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }
}
