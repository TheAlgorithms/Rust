// 1-D Array Peak Finder
pub fn peak_finder_1d_greedy(arr: &[usize]) -> Option<usize> {
    for (i, value) in arr.iter().enumerate() {
        if arr.len() == 1
            || (i == 0 && *value >= arr[i + 1])
            || (i > 0 && i < (arr.len() - 1) && (arr[i - 1] <= *value && *value >= arr[i + 1])
                || (i == arr.len() - 1 && *value >= arr[i - 1]))
        {
            return Some(*value);
        } else {
            continue;
        }
    }
    None
}

pub fn peak_finder_1d_divide_conquer(arr: &[usize]) -> Option<usize> {
    let mut _arr = vec![0; arr.len()];
    _arr.clone_from_slice(arr);
    let mid_index = arr.len() / 2;
    if arr.is_empty() {
        return None;
    }
    if mid_index > 0 && _arr[mid_index] <= _arr[mid_index - 1] {
        peak_finder_1d_divide_conquer(_arr.split_at(mid_index).0)
    } else if mid_index < (_arr.len() - 1) && _arr[mid_index] <= _arr[mid_index + 1] {
        peak_finder_1d_divide_conquer(_arr.split_at(mid_index).1)
    } else {
        Some(_arr[mid_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_peak_using_loop() {
        let mut peak = peak_finder_1d_greedy(&[]);
        assert_eq!(peak, None);
        peak = peak_finder_1d_greedy(&[6, 7, 5, 4, 3, 2, 1, 4, 5]);
        assert_eq!(peak, Some(7));
        peak = peak_finder_1d_greedy(&[6, 7, 8]);
        assert_eq!(peak, Some(8));
    }

    #[test]
    fn find_peak_using_divide_conquer() {
        let mut peak = peak_finder_1d_divide_conquer(&[]);
        assert_eq!(peak, None);
        peak = peak_finder_1d_divide_conquer(&[6, 7, 5, 4, 3, 2, 1, 4, 5]);
        assert_eq!(peak, Some(7));
        peak = peak_finder_1d_divide_conquer(&[6, 7, 8]);
        assert_eq!(peak, Some(8));
        peak = peak_finder_1d_divide_conquer(&[9, 7, 8]);
        assert_eq!(peak, Some(9));
    }
}
