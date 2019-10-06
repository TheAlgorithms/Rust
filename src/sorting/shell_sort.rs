pub fn shell_sort<T: Ord + Copy>(values: &mut Vec<T>) {
    // shell sort works by swiping the value at a given gap and decreasing the gap to 1
    fn insertion<T: Ord + Copy>(values: &mut Vec<T>, start: usize, gap: usize) {
        for i in ((start + gap)..values.len()).step_by(gap) {
            let val_current = values[i];
            let mut pos = i;
            // make swaps
            while pos >= gap && values[pos - gap] > val_current {
                values[pos] = values[pos - gap];
                pos = pos - gap;
            }
            values[pos] = val_current;
        }
    }

    let mut count_sublist = values.len() / 2; // makes gap as long as half of the array
    while count_sublist > 0 {
        for pos_start in 0..count_sublist {
            insertion(values, pos_start, count_sublist);
        }
        count_sublist /= 2; // makes gap as half of previous
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let mut vec = vec![3, 5, 6, 3, 1, 4];
        shell_sort(&mut ve3);
        for i in 0..ve3.len() - 1 {
            assert!(ve3[i] <= ve3[i + 1]);
        }
    }

    #[test]
    fn empty() {
        let mut vec: Vec<i32> = vec![];
        shell_sort(&mut vec);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn reverse() {
        let mut vec = vec![6, 5, 4, 3, 2, 1];
        shell_sort(&mut ve1);
        for i in 0..ve1.len() - 1 {
            assert!(ve1[i] <= ve1[i + 1]);
        }
    }

    #[test]
    fn already_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        shell_sort(&mut ve2);
        for i in 0..ve2.len() - 1 {
            assert!(ve2[i] <= ve2[i + 1]);
        }
    }
}
