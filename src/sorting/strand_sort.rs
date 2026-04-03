pub fn strand_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let mut result: Vec<T> = Vec::new();
    let mut input: Vec<T> = arr.to_vec();

    while !input.is_empty() {
        let mut sublist: Vec<T> = Vec::new();
        let mut leftovers: Vec<T> = Vec::with_capacity(input.len());

        for item in input {
            if sublist.is_empty() || item >= *sublist.last().unwrap() {
                sublist.push(item);
            } else {
                leftovers.push(item);
            }
        }

        result = merge(result, sublist);
        input = leftovers;
    }

    arr.clone_from_slice(&result);
}

fn merge<T: Ord + Clone>(left: Vec<T>, right: Vec<T>) -> Vec<T> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let mut left_iter = left.into_iter().peekable();
    let mut right_iter = right.into_iter().peekable();

    while let (Some(l), Some(r)) = (left_iter.peek(), right_iter.peek()) {
        if l <= r {
            merged.push(left_iter.next().unwrap());
        } else {
            merged.push(right_iter.next().unwrap());
        }
    }

    merged.extend(left_iter);
    merged.extend(right_iter);

    merged
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        let cloned = res.clone();
        strand_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        let cloned = res.clone();
        strand_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        let cloned = res.clone();
        strand_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        let cloned = res.clone();
        strand_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        let cloned = res.clone();
        strand_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        let cloned = res.clone();
        strand_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn duplicates() {
        let mut res = vec![4, 2, 4, 3, 2, 1, 4];
        let cloned = res.clone();
        strand_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }
}
