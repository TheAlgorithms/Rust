pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        // Already sorted
        return;
    }
    let length = arr.len();
    let mid = length/2;

    _merge_sort(arr, 0, mid);
    _merge_sort(arr, mid, length);
    _merge(arr, 0, mid, length);
}

// Merge Sort
// Takes arguments `left` and `right` -> indexes into `arr` in the range of
// [left, right), where `left` <= [range] < `right`
// (right is not included in the range).
fn _merge_sort<T: Ord + Copy>(arr: &mut [T], left: usize, right: usize) {
    if left < (right-1) {
        let mid = (left+right)/2;
        _merge_sort(arr, left, mid);
        _merge_sort(arr, mid, right);
        _merge(arr, left, mid, right);
    }
}

// _merge()
// `left`  - Index of the first value in the left range
// `mid`   - Index of the first value in the right range
// `right` - [Index + 1] of the last value in the right range
fn _merge<T: Ord + Copy>(arr: &mut [T], left: usize, mid: usize, right: usize) {
    // Need to copy values to temp arrays so we don't overwrite any values
    let mut left_vect  = Vec::new();
    let mut right_vect = Vec::new();

    for idx in left..mid {
        left_vect.push(arr[idx].clone());
    }
    for idx in mid..right {
        right_vect.push(arr[idx].clone());
    }

    let mut idx = left;
    let mut lv = 0;
    let mut rv = 0;
    while lv < left_vect.len() && rv < right_vect.len() {
        if right_vect[rv] < left_vect[lv] {
            arr[idx] = right_vect[rv];
            rv += 1;
        }
        else {
            arr[idx] = left_vect[lv];
            lv += 1;
        }
        idx += 1;
    }

    while lv < left_vect.len()  { arr[idx] = left_vect[lv];  lv+=1; idx+=1; }
    while rv < right_vect.len() { arr[idx] = right_vect[rv]; rv+=1; idx+=1; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_sort_test_evens() {
        let mut arr = [23, 4, 2, 8, 0, -7, 32, -6, 16, 556];
        merge_sort(&mut arr);
        assert_eq!(arr, [-7, -6, 0, 2, 4, 8, 16, 23, 32, 556]);

        let mut arr = [16, 8, 15, 42, 23, 4];
        merge_sort(&mut arr);
        assert_eq!(arr, [4, 8, 15, 16, 23, 42]);
    }
    #[test]
    fn merge_sort_test_odds() {
        let mut arr = [23, 4, 2, 8, 0, 84, -7, 32, -6, 16, 556];
        merge_sort(&mut arr);
        assert_eq!(arr, [-7, -6, 0, 2, 4, 8, 16, 23, 32, 84, 556]);

        let mut arr = [16, 8, 15, 42, 0, 23, 4];
        merge_sort(&mut arr);
        assert_eq!(arr, [0, 4, 8, 15, 16, 23, 42]);    }
    #[test]
    fn merge_sort_test_in_order() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7, 8];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);

        let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn merge_sort_test_backwards_order() {
        let mut arr = [8, 7, 6, 5, 4, 3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);

        let mut arr = [9, 8, 7, 6, 5, 4, 3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn merge_sort_edge_cases() {
        let mut arr = [1];
        merge_sort(&mut arr);
        assert_eq!(arr, [1]);

        let mut arr = [1, 2, 1, 4, 1, 4, 3, 4, 1, 4];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 1, 1, 1, 2, 3, 4, 4, 4, 4]);

        let mut arr = [5, 5, 5, 5, 5];
        merge_sort(&mut arr);
        assert_eq!(arr, [5, 5, 5, 5, 5]);

        let mut arr = ["R", "U", "S", "T"];
        merge_sort(&mut arr);
        assert_eq!(arr, ["R", "S", "T", "U"]);
    }
}
