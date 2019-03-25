use std::cmp;

#[allow(dead_code)]
pub fn insertion_sort<T>(arr: &[T]) -> Vec<T>
where
    T: cmp::PartialEq + cmp::PartialOrd + Clone,
{
    // The resulting vector should contain the same amount of elements as
    // the slice that is being sorted, so enough room is preallocated
    let mut result: Vec<T> = Vec::with_capacity(arr.len());

    // Iterate over the elements to sort and
    // put a clone of the element to insert in elem.
    for elem in arr.iter().cloned() {
        // How many elements have already been inserted?
        let n_inserted = result.len();

        // Loop over the inserted elements and one more index.
        for i in 0..=n_inserted {
            // If at the end or result[i] is larger than the current element,
            // we have found the right spot:
            if i == n_inserted || result[i] > elem {
                // Insert the element at i,
                // move the rest to higher indexes:
                result.insert(i, elem);
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let res = insertion_sort(&Vec::<u8>::new());
        assert_eq!(res, vec![]);
    }

    #[test]
    fn one_element() {
        let res = insertion_sort(&vec!["a"]);
        assert_eq!(res, vec!["a"]);
    }

    #[test]
    fn already_sorted() {
        let res = insertion_sort(&vec!["a", "b", "c"]);
        assert_eq!(res, vec!["a", "b", "c"]);
    }

    #[test]
    fn basic() {
        let res = insertion_sort(&vec!["d", "a", "c", "b"]);
        assert_eq!(res, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn odd_number_of_elements() {
        let res = insertion_sort(&vec!["d", "a", "c", "e", "b"]);
        assert_eq!(res, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn repeated_elements() {
        let res = insertion_sort(&vec![542, 542, 542, 542]);
        assert_eq!(res, vec![542, 542, 542, 542]);
    }
}
