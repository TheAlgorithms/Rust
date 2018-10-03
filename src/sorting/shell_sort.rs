pub fn shell_sort<T: Ord+Copy>(values: &mut Vec<T>) {
    fn sort_gap_insertation<T: Ord+Copy>(values: &mut Vec<T>, start: usize, gap: usize) {
        for i in ((start + gap)..values.len()).step_by(gap) {
            let val_current = values[i];
            let mut pos = i;

            while pos >= gap && values[pos - gap] > val_current {
                values[pos] = values[pos - gap];
                pos = pos - gap;
            }
            values[pos] = val_current;
        }
    }

    let mut count_sublist = values.len() / 2;
    while count_sublist > 0 {
        for pos_start in 0..count_sublist {
            sort_gap_insertation(values, pos_start, count_sublist);
        }
        count_sublist /= 2;
    }

}