pub fn longest_increasing_subsequence<T: PartialOrd + Copy>(list: &[T]) -> Vec<T> {
    longest_increasing_subsequence_by(list, |a, b| a <= b)
}

pub fn longest_strictly_increasing_subsequence<T: PartialOrd + Copy>(list: &[T]) -> Vec<T> {
    longest_increasing_subsequence_by(list, |a, b| a < b)
}

// pseudocode from https://en.wikipedia.org/wiki/Longest_increasing_subsequence#Efficient_algorithms
pub fn longest_increasing_subsequence_by<T: Copy, F>(list: &[T], comp: F) -> Vec<T>
where
    F: Fn(&T, &T) -> bool,
{
    let length = list.len();

    let mut predecessor_index = Vec::with_capacity(length);
    let mut smallest_value_index: Vec<usize> = std::iter::repeat(0).take(length + 1).collect();

    let mut l = 0;
    for i in 0..length {
        // Binary search for the largest positive j ≤ l
        // such that list[smallest_value_index[j]] < list[i]
        let mut lo = 1;
        let mut hi = l;
        while lo <= hi {
            let mid = (lo + hi + 1) / 2;
            if comp(&list[smallest_value_index[mid]], &list[i]) {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        // After searching, lo is 1 greater than the
        // length of the longest prefilist of list[i]
        let new_l = lo;

        // The predecessor of list[i] is the last index of
        // the subsequence of length new_l-1
        predecessor_index.push(smallest_value_index[new_l - 1]);
        smallest_value_index[new_l] = i;

        if new_l > l {
            // If we found a subsequence longer than any we've
            // found yet, update l
            l = new_l;
        }
    }

    // Reconstruct the longest increasing subsequence
    let mut s = Vec::with_capacity(l);
    let mut k = smallest_value_index[l];
    for _ in 0..l {
        s.push(list[k]);
        k = predecessor_index[k];
    }

    s.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vec() {
        let index = longest_increasing_subsequence::<usize>(&vec![]);
        assert_eq!(index, vec![]);
    }

    #[test]
    fn single_element() {
        let index = longest_increasing_subsequence(&vec![0]);
        assert_eq!(index, vec![0]);
    }

    #[test]
    fn all_increasing() {
        let list = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let index = longest_increasing_subsequence(&list);
        assert_eq!(index, list);
    }

    #[test]
    fn all_decreasing() {
        let index = longest_increasing_subsequence(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
        assert_eq!(index.len(), 1);
    }

    #[test]
    // multiple answers
    fn random_list() {
        let index = longest_increasing_subsequence(&vec![
            0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15,
        ]);
        assert_eq!(index.len(), 6);
    }

    #[test]
    fn random_list2() {
        let index = longest_increasing_subsequence(&vec![0, 2, 3, 5, 7, 4, 5, 6]);
        assert_eq!(index, vec![0, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn random_list3() {
        let data = vec![1, 3, 5, 7, 9, 4];
        assert_eq!(vec![1, 3, 5, 7, 9], longest_increasing_subsequence(&data));
    }

    #[test]
    fn longest_decreasing_subsequence() {
        let list = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let index = longest_increasing_subsequence_by(&list, |a, b| a > b);
        assert_eq!(index, list);
    }
}
