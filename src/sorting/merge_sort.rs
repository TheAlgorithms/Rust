fn merge<T: Ord + Copy>(arr: &mut [T], mid: usize, scratch: &mut [T]) {
    // The snapshot must survive writes to arr; reuse its storage across merges.
    let snapshot = &mut scratch[..arr.len()];
    snapshot.copy_from_slice(arr);
    let (left_half, right_half) = snapshot.split_at(mid);
    let mut left = 0;
    let mut right = 0;

    for value in arr {
        // Preserve the original tie rule: equal elements come from the right.
        if right == right_half.len()
            || (left < left_half.len() && left_half[left] < right_half[right])
        {
            *value = left_half[left];
            left += 1;
        } else {
            *value = right_half[right];
            right += 1;
        }
    }
}

pub fn top_down_merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let mut scratch = arr.to_vec();
    sort_recursive(arr, &mut scratch);
}

fn sort_recursive<T: Ord + Copy>(arr: &mut [T], scratch: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);
    sort_recursive(left, scratch);
    sort_recursive(right, scratch);
    merge(arr, mid, scratch);
}

pub fn bottom_up_merge_sort<T: Copy + Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let mut scratch = arr.to_vec();
    let mut width = 1;
    while width < arr.len() {
        let mut start = 0;
        while arr.len() - start > width {
            let end = (start + 2 * width).min(arr.len());
            merge(&mut arr[start..end], width, &mut scratch);
            start = end;
        }
        width *= 2;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exhaustive_small_inputs_match_standard_sort() {
        for len in 0..=8 {
            for mut encoded in 0..3_usize.pow(len) {
                let input: Vec<i32> = (0..len)
                    .map(|_| {
                        let value = (encoded % 3) as i32 - 1;
                        encoded /= 3;
                        value
                    })
                    .collect();
                let mut expected = input.clone();
                expected.sort();

                for sort in [
                    super::top_down_merge_sort::<i32>,
                    super::bottom_up_merge_sort::<i32>,
                ] {
                    let mut actual = input.clone();
                    sort(&mut actual);
                    assert_eq!(actual, expected);
                }
            }
        }
    }

    #[test]
    fn merge_boundaries_match_standard_sort() {
        for len in [9, 15, 16, 17, 31, 32, 33, 255, 256, 257, 1023, 1024, 1025] {
            let mut input: Vec<i32> = (0..len).map(|index| (index * 31 % 17) - 8).collect();
            let mut expected = input.clone();
            expected.sort();

            for _ in 0..2 {
                for sort in [
                    super::top_down_merge_sort::<i32>,
                    super::bottom_up_merge_sort::<i32>,
                ] {
                    let mut actual = input.clone();
                    sort(&mut actual);
                    assert_eq!(actual, expected);
                }
                input.reverse();
            }
        }
    }

    #[test]
    fn equal_elements_keep_right_first_order() {
        let input = [2, 1, 2, 1, 3, 2, 3];
        // A stable sort of reversed references models the original right-first
        // tie rule. Distinct addresses let us observe equal elements' order.
        let mut expected: Vec<_> = input.iter().rev().collect();
        expected.sort();

        for sort in [
            super::top_down_merge_sort::<&i32>,
            super::bottom_up_merge_sort::<&i32>,
        ] {
            let mut actual: Vec<_> = input.iter().collect();
            sort(&mut actual);
            for (actual, expected) in actual.iter().zip(&expected) {
                assert!(std::ptr::eq(*actual, *expected));
            }
        }
    }

    #[cfg(test)]
    mod top_down_merge_sort {
        use super::super::*;
        use crate::sorting::have_same_elements;
        use crate::sorting::is_sorted;

        #[test]
        fn basic() {
            let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn basic_string() {
            let mut res = vec!["a", "bb", "d", "cc"];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn empty() {
            let mut res = Vec::<u8>::new();
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn one_element() {
            let mut res = vec![1];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn pre_sorted() {
            let mut res = vec![1, 2, 3, 4];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn reverse_sorted() {
            let mut res = vec![4, 3, 2, 1];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }
    }

    #[cfg(test)]
    mod bottom_up_merge_sort {
        use super::super::*;
        use crate::sorting::have_same_elements;
        use crate::sorting::is_sorted;

        #[test]
        fn basic() {
            let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn basic_string() {
            let mut res = vec!["a", "bb", "d", "cc"];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn empty() {
            let mut res = Vec::<u8>::new();
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn one_element() {
            let mut res = vec![1];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn pre_sorted() {
            let mut res = vec![1, 2, 3, 4];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn reverse_sorted() {
            let mut res = vec![4, 3, 2, 1];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }
    }
}
