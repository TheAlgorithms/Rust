/// From Wikipedia:
/// Tournament sort is a sorting algorithm. It improves upon the naive
/// selection sort by using a priority queue to find the next element in
/// the sort.
///
/// Time complexity is `O(n log n)`, where `n` is the number of elements.
/// Space complexity is `O(n)`.
pub fn tournament_sort(arr: &[i32]) -> Vec<i32> {
    let mut arr = arr.to_vec();
    let n = arr.len();
    let mut tree_size = 1;

    while tree_size < n {
        tree_size <<= 1;
    }

    let mut tree: Vec<i32> = vec![0; 2 * tree_size];

    for i in 0..tree_size {
        if i < n {
            tree[tree_size + i] = arr[i];
        } else {
            tree[tree_size + i] = i32::MAX;
        }
    }

    for i in (1..tree_size).rev() {
        tree[i] = tree[2 * i].min(tree[2 * i + 1]);
    }

    for i in 0..n {
        let min = tree[1];
        arr[i] = min;

        let mut pos = 1;
        while pos < tree_size {
            if tree[2 * pos] == min {
                pos *= 2;
            } else {
                pos = 2 * pos + 1;
            }
        }

        tree[pos] = i32::MAX;
        while pos > 1 {
            pos >>= 1;
            tree[pos] = tree[2 * pos].min(tree[2 * pos + 1]);
        }
    }
    arr
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn descending() {
        let arr = vec![6, 5, 4, 3, 2, 1];
        let res = tournament_sort(&arr);
        assert!(is_sorted(&res) && have_same_elements(&res, &arr));
    }

    #[test]
    fn empty() {
        let arr = Vec::<i32>::new();
        let res = tournament_sort(&arr);
        assert!(is_sorted(&res) && have_same_elements(&res, &arr));
    }

    #[test]
    fn negative_numbers() {
        let arr = vec![-32, -54, -65, -12, -7];
        let res = tournament_sort(&arr);
        assert!(is_sorted(&res) && have_same_elements(&res, &arr));
    }

    #[test]
    fn one_element() {
        let arr = vec![1];
        let res = tournament_sort(&arr);
        assert!(is_sorted(&res) && have_same_elements(&res, &arr));
    }

    #[test]
    fn pre_sorted() {
        let arr = vec![5, 12, 23, 54, 57, 60];
        let res = tournament_sort(&arr);
        assert!(is_sorted(&res) && have_same_elements(&res, &arr));
    }

    #[test]
    fn repeated_elements() {
        let arr = vec![42, 42, 42, 42];
        let res = tournament_sort(&arr);
        assert!(is_sorted(&res) && have_same_elements(&res, &arr));
    }
}
