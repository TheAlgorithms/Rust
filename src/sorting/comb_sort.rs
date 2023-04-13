pub fn comb_sort<T: Ord>(arr: &mut [T]) {
    let mut gap = arr.len();
    let shrink = 1.3;
    let mut sorted = false;

    while !sorted {
        gap = (gap as f32 / shrink).floor() as usize;
        if gap <= 1 {
            gap = 1;
            sorted = true;
        }
        for i in 0..arr.len() - gap {
            let j = i + gap;
            if arr[i] > arr[j] {
                arr.swap(i, j);
                sorted = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn descending() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        let cloned = ve1.clone();
        comb_sort(&mut ve1);
        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        let cloned = ve2.clone();
        comb_sort(&mut ve2);
        assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }

    #[test]
    fn duplicates() {
        //pre-sorted
        let mut ve3 = vec![2, 2, 2, 2, 2, 1];
        let cloned = ve3.clone();
        comb_sort(&mut ve3);
        assert!(is_sorted(&ve3) && have_same_elements(&ve3, &cloned));
    }
}
