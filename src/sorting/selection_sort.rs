pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();

    if len < 2 {
        return;
    }

    for left in 0..(len - 1) {
        let mut smallest = left;

        for right in (left + 1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }

        if left != smallest {
            arr.swap(left, smallest);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn basic() {
        let mut res = vec!["d", "a", "c", "b"];
        let cloned = res.clone();
        selection_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        let cloned = res.clone();
        selection_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn one_element() {
        let mut res = vec!["a"];
        let cloned = res.clone();
        selection_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec!["a", "b", "c"];
        let cloned = res.clone();
        selection_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }
}
