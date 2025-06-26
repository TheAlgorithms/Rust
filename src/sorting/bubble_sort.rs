/// This function works only with integers
/// For ordering float values use https://crates.io/crates/float-ord
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    let mut sorted = false;
    let mut n = arr.len();
    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        n -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;
    use rand::rng;
    use rand::seq::SliceRandom;

    #[test]
    fn descending() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        let cloned = ve1.clone();
        bubble_sort(&mut ve1);
        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        let cloned = ve2.clone();
        bubble_sort(&mut ve2);
        assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }
    #[test]
    fn empty() {
        let mut ve3: Vec<usize> = vec![];
        let cloned = ve3.clone();
        bubble_sort(&mut ve3);
        assert!(is_sorted(&ve3) && have_same_elements(&ve3, &cloned));
    }
    #[test]
    fn duplicate() {
        let mut ve3: Vec<usize> = vec![2, 3, 3, 5, 7, 1, 1];
        let cloned = ve3.clone();
        bubble_sort(&mut ve3);
        assert!(is_sorted(&ve3) && have_same_elements(&ve3, &cloned));
    }
    #[test]
    fn negative() {
        let mut ve3: Vec<i32> = (-10..-1).rev().collect();
        let cloned = ve3.clone();
        bubble_sort(&mut ve3);
        assert!(is_sorted(&ve3) && have_same_elements(&ve3, &cloned));
    }
    #[test]
    fn withnull() {
        let mut ve3 = vec![2, 3, -3, 5, 7, 0, 1];
        let cloned = ve3.clone();
        bubble_sort(&mut ve3);
        assert!(is_sorted(&ve3) && have_same_elements(&ve3, &cloned));
    }
    #[test]
    fn long() {
        let mut ve3: Vec<i32> = (10..39000).collect();
        ve3.shuffle(&mut rng());
        let cloned = ve3.clone();
        bubble_sort(&mut ve3);
        assert!(is_sorted(&ve3) && have_same_elements(&ve3, &cloned));
    }
}
