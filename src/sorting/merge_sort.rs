/// Merge Sort Algorithm
/// Works for all Clone + Copy types (like primitive numbers)

// Merge Sort
pub fn merge_sort<T>(values: &[T]) -> Vec<T>
where
    T: PartialOrd + Clone + Copy,
{
    if values.len() <= 1 {
        values.to_vec()
    } else {
        let mid = values.len() / 2;
        merge(&merge_sort(&values[..mid]), &merge_sort(&values[mid..]))
    }
}

// Merge two sorted slices
// This is where the magic happens when merge_sort above is called
// recursively until a starting slice is broken down into N Vecs of single items
fn merge<T>(left: &[T], right: &[T]) -> Vec<T>
where
    T: PartialOrd + Clone + Copy,
{
    // The destination Vec to contain all items (sorted)
    let mut merged = Vec::with_capacity(left.len() + right.len());
    // Vec.remove(0) is expensive because it shifts all remaining
    // items left. Instead, keep a cursor of position for each list
    let (mut left_idx, mut right_idx) = (0, 0);
    // Loop as long as each of the lists have remaining items
    while (left_idx < left.len()) && (right_idx < right.len()) {
        if &left[left_idx] < &right[right_idx] {
            merged.push(left[left_idx]);
            left_idx += 1;
        } else {
            merged.push(right[right_idx]);
            right_idx += 1;
        }
    }
    // Check to see which list has remaining items
    if left_idx < left.len() {
        merged.extend(&left[left_idx..]);
    } else if right_idx < right.len() {
        merged.extend(&right[right_idx..]);
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn test_merge_empty() {
        let empty: Vec<u32> = Vec::new();
        let result = merge_sort(&empty);
        assert!(is_sorted(&result));
    }
    #[test]
    fn test_merge_single() {
        let result = merge_sort(&vec![5]);
        assert!(is_sorted(&result));
    }
    #[test]
    fn test_merge_sort() {
        let values = vec![3, 1, 12, 4, 9, 2];
        let result = merge_sort(&values);
        assert!(is_sorted(&result));
    }
    #[test]
    fn test_merge_already_sorted() {
        let values = vec![1, 2, 3, 4, 9, 12];
        let result = merge_sort(&values);
        assert!(is_sorted(&result));
    }

    #[test]
    fn test_merge_sort_with_negative() {
        let values = vec![3, 1, 12, -2, 4, 9, 2, -10];
        let result = merge_sort(&values);
        assert!(is_sorted(&result));
    }
}
