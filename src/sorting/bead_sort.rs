//Bead sort only works for sequences of non-negative integers.
//https://en.wikipedia.org/wiki/Bead_sort
pub fn bead_sort(a: &mut [usize]) {
    // Find the maximum element
    let mut max = a[0];
    (1..a.len()).for_each(|i| {
        if a[i] > max {
            max = a[i];
        }
    });

    // allocating memory
    let mut beads = vec![vec![0; max]; a.len()];

    // mark the beads
    for i in 0..a.len() {
        for j in (0..a[i]).rev() {
            beads[i][j] = 1;
        }
    }

    // move down the beads
    for j in 0..max {
        let mut sum = 0;
        (0..a.len()).for_each(|i| {
            sum += beads[i][j];
            beads[i][j] = 0;
        });

        for k in ((a.len() - sum)..a.len()).rev() {
            a[k] = j + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descending() {
        //descending
        let mut ve1: [usize; 5] = [5, 4, 3, 2, 1];
        bead_sort(&mut ve1);
        assert!(crate::sorting::is_sorted(&ve1));
    }

    #[test]
    fn mix_values() {
        //pre-sorted
        let mut ve2: [usize; 5] = [7, 9, 6, 2, 3];
        bead_sort(&mut ve2);
        assert!(crate::sorting::is_sorted(&ve2));
    }
}
