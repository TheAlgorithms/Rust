fn merge<T: Ord + Copy>(arr: &mut [T], mid: usize) {
    // Create temporary vectors to support the merge.
    let left_half = arr[..mid].to_vec();
    let right_half = arr[mid..].to_vec();

    // Indexes to track the positions while merging.
    let mut l = 0;
    let mut r = 0;

    for v in arr {
        // Choose either the smaller element, or from whichever vec is not exhausted.
        if r == right_half.len() || (l < left_half.len() && left_half[l] < right_half[r]) {
            *v = left_half[l];
            l += 1;
        } else {
            *v = right_half[r];
            r += 1;
        }
    }
}

pub fn top_down_merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        // Sort the left half recursively.
        top_down_merge_sort(&mut arr[..mid]);
        // Sort the right half recursively.
        top_down_merge_sort(&mut arr[mid..]);
        // Combine the two halves.
        merge(arr, mid);
    }
}

pub fn bottom_up_merge_sort<T: Copy + Ord>(a: &mut [T]) {
    if a.len() > 1 {
        let len: usize = a.len();
        let mut sub_array_size: usize = 1;
        while sub_array_size < len {
            let mut start_index: usize = 0;
            // still have more than one sub-arrays to merge
            while len - start_index > sub_array_size {
                let end_idx: usize = if start_index + 2 * sub_array_size > len {
                    len
                } else {
                    start_index + 2 * sub_array_size
                };
                // merge a[start_index..start_index+sub_array_size] and a[start_index+sub_array_size..end_idx]
                // NOTE: mid is a relative index number starting from `start_index`
                merge(&mut a[start_index..end_idx], sub_array_size);
                // update `start_index` to merge the next sub-arrays
                start_index = end_idx;
            }
            sub_array_size *= 2;
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod top_down_merge_sort {
        use super::super::*;

        #[test]
        fn basic() {
            let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
            top_down_merge_sort(&mut res);
            assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        }

        #[test]
        fn basic_string() {
            let mut res = vec!["a", "bb", "d", "cc"];
            top_down_merge_sort(&mut res);
            assert_eq!(res, vec!["a", "bb", "cc", "d"]);
        }

        #[test]
        fn empty() {
            let mut res = Vec::<u8>::new();
            top_down_merge_sort(&mut res);
            assert_eq!(res, vec![]);
        }

        #[test]
        fn one_element() {
            let mut res = vec![1];
            top_down_merge_sort(&mut res);
            assert_eq!(res, vec![1]);
        }

        #[test]
        fn pre_sorted() {
            let mut res = vec![1, 2, 3, 4];
            top_down_merge_sort(&mut res);
            assert_eq!(res, vec![1, 2, 3, 4]);
        }

        #[test]
        fn reverse_sorted() {
            let mut res = vec![4, 3, 2, 1];
            top_down_merge_sort(&mut res);
            assert_eq!(res, vec![1, 2, 3, 4]);
        }
    }

    #[cfg(test)]
    mod bottom_up_merge_sort {
        use super::super::*;

        #[test]
        fn basic() {
            let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
            bottom_up_merge_sort(&mut res);
            assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        }

        #[test]
        fn basic_string() {
            let mut res = vec!["a", "bb", "d", "cc"];
            bottom_up_merge_sort(&mut res);
            assert_eq!(res, vec!["a", "bb", "cc", "d"]);
        }

        #[test]
        fn empty() {
            let mut res = Vec::<u8>::new();
            bottom_up_merge_sort(&mut res);
            assert_eq!(res, vec![]);
        }

        #[test]
        fn one_element() {
            let mut res = vec![1];
            bottom_up_merge_sort(&mut res);
            assert_eq!(res, vec![1]);
        }

        #[test]
        fn pre_sorted() {
            let mut res = vec![1, 2, 3, 4];
            bottom_up_merge_sort(&mut res);
            assert_eq!(res, vec![1, 2, 3, 4]);
        }

        #[test]
        fn reverse_sorted() {
            let mut res = vec![4, 3, 2, 1];
            bottom_up_merge_sort(&mut res);
            assert_eq!(res, vec![1, 2, 3, 4]);
        }
    }
}
