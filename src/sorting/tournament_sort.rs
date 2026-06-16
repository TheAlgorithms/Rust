/// From Wikipedia:
/// Tournament sort is a sorting algorithm. It improves upon the naive
/// selection sort by using a priority queue to find the next element in
/// the sort.
///
/// Time complexity is `O(n log n)`, where `n` is the number of elements.
/// Space complexity is `O(n)`.
pub fn tournament_sort<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    let mut arr = arr.to_vec();
    let n = arr.len();
    let mut tree_size = 1;

    while tree_size < n {
        tree_size <<= 1;
    }

    let mut tree: Vec<Option<T>> = vec![None; 2 * tree_size];

    for i in 0..tree_size {
        if i < n {
            tree[tree_size + i] = Some(arr[i].clone());
        } else {
            tree[tree_size + i] = None;
        }
    }

    for i in (1..tree_size).rev() {
        tree[i] = min_opt(&tree[2 * i], &tree[2 * i + 1]);
    }

    for i in 0..n {
        let min = tree[1].clone().unwrap();
        arr[i] = min.clone();

        let mut pos = 1;
        while pos < tree_size {
            if tree[2 * pos].as_ref() == Some(&min) {
                pos *= 2;
            } else {
                pos = 2 * pos + 1;
            }
        }

        tree[pos] = None;
        while pos > 1 {
            pos >>= 1;
            tree[pos] = min_opt(&tree[2 * pos], &tree[2 * pos + 1]);
        }
    }
    arr
}

fn min_opt<T: Ord + Clone>(a: &Option<T>, b: &Option<T>) -> Option<T> {
    match (a, b) {
        (Some(x), Some(y)) => Some(if x <= y { x.clone() } else { y.clone() }),
        (Some(x), None) => Some(x.clone()),
        (None, Some(y)) => Some(y.clone()),
        (None, None) => None,
    }
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
        assert_eq!(res, { let mut expected = arr.clone(); expected.sort(); expected });
    }
}
