// sorts with the minimum number of rewrites. Runs through all values in the array, placing them in their correct spots. O(n^2).

pub fn cycle_sort(arr: &mut [i32]) {
    for cycle_start in 0..arr.len() {
        let mut item = arr[cycle_start];
        let mut pos = cycle_start;
        for i in arr.iter().skip(cycle_start + 1) {
            if *i < item {
                pos += 1;
            }
        }
        if pos == cycle_start {
            continue;
        }
        while item == arr[pos] {
            pos += 1;
        }
        std::mem::swap(&mut arr[pos], &mut item);
        while pos != cycle_start {
            pos = cycle_start;
            for i in arr.iter().skip(cycle_start + 1) {
                if *i < item {
                    pos += 1;
                }
            }
            while item == arr[pos] {
                pos += 1;
            }
            std::mem::swap(&mut arr[pos], &mut item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;
    #[test]
    fn it_works() {
        let mut arr1 = [6, 5, 4, 3, 2, 1];
        cycle_sort(&mut arr1);
        assert!(is_sorted(&arr1));
        arr1 = [12, 343, 21, 90, 3, 21];
        cycle_sort(&mut arr1);
        assert!(is_sorted(&arr1));
        let mut arr2 = [1];
        cycle_sort(&mut arr2);
        assert!(is_sorted(&arr2));
        let mut arr3 = [213, 542, 90, -23412, -32, 324, -34, 3324, 54];
        cycle_sort(&mut arr3);
        assert!(is_sorted(&arr3));
    }
}
