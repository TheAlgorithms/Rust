use std::cmp;

/// Sorts a mutable slice using in-place insertion sort algorithm.
///
/// Time complexity is `O(n^2)`, where `n` is the number of elements.
/// Space complexity is `O(1)` as it sorts elements in-place.
pub fn insertion_sort<T>(arr: &mut [T])
where
    T: cmp::PartialOrd + Copy,
{
    for i in 1..arr.len() {
        let cur = arr[i];
        let mut j = i - 1;

        // This loop has two exit points, and needs to be handled differently.
        // Exit point1
        while arr[j] > cur {
            arr[j + 1] = arr[j];
            if j == 0 {
                // Exit point2
                break;
            }
            j -= 1;
        }

        // Exits from "Exit point2"
        // This `&& arr[0] > cur` is needed here as there is a possibility that
        // after executing `j -= 1;`, `j` becomes 0 and then `arr[j] > cur` is
        // not satisfied so that we exit from "Exit point1" with `j` equals 0.
        if j == 0 && arr[0] > cur {
            arr[0] = cur;
        } else {
            // Exits from "exit point1" with condition not satisfied
            arr[j + 1] = cur;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn empty() {
        let mut arr: [u8; 0] = [];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn one_element() {
        let mut arr: [char; 1] = ['a'];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn already_sorted() {
        let mut arr: [&str; 3] = ["a", "b", "c"];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn basic() {
        let mut arr: [&str; 4] = ["d", "a", "c", "b"];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr: Vec<&str> = vec!["d", "a", "c", "e", "b"];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn repeated_elements() {
        let mut arr: Vec<usize> = vec![542, 542, 542, 542];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }
}
