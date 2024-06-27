use std::cmp::{max, min};

// Function for finding the maximum and minimum element of the Array
fn max_min(vec: &[i32], bingo: &mut i32, next_bingo: &mut i32) {
    for &element in vec.iter().skip(1) {
        *bingo = min(*bingo, element);
        *next_bingo = max(*next_bingo, element);
    }
}

pub fn bingo_sort(vec: &mut [i32]) {
    if vec.is_empty() {
        return;
    }

    let mut bingo = vec[0];
    let mut next_bingo = vec[0];

    max_min(vec, &mut bingo, &mut next_bingo);

    let largest_element = next_bingo;
    let mut next_element_pos = 0;

    for (bingo, _next_bingo) in (bingo..=largest_element).zip(bingo..=largest_element) {
        let start_pos = next_element_pos;

        for i in start_pos..vec.len() {
            if vec[i] == bingo {
                vec.swap(i, next_element_pos);
                next_element_pos += 1;
            }
        }
    }
}

#[allow(dead_code)]
fn print_array(arr: &[i32]) {
    print!("Sorted Array: ");
    for &element in arr {
        print!("{element} ");
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bingo_sort() {
        let mut arr = vec![5, 4, 8, 5, 4, 8, 5, 4, 4, 4];
        bingo_sort(&mut arr);
        assert_eq!(arr, vec![4, 4, 4, 4, 4, 5, 5, 5, 8, 8]);

        let mut arr2 = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        bingo_sort(&mut arr2);
        assert_eq!(arr2, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut arr3 = vec![0, 1, 0, 1, 0, 1];
        bingo_sort(&mut arr3);
        assert_eq!(arr3, vec![0, 0, 0, 1, 1, 1]);
    }

    #[test]
    fn test_empty_array() {
        let mut arr = Vec::new();
        bingo_sort(&mut arr);
        assert_eq!(arr, Vec::new());
    }

    #[test]
    fn test_single_element_array() {
        let mut arr = vec![42];
        bingo_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut arr = vec![-5, -4, -3, -2, -1];
        bingo_sort(&mut arr);
        assert_eq!(arr, vec![-5, -4, -3, -2, -1]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        bingo_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        bingo_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5];
        bingo_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
    }
}
