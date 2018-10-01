use std::cmp;
use std::fmt;

pub fn insertion_sort<T>(arr: &[T]) -> Vec<T>
where
    T: cmp::PartialEq + cmp::PartialOrd + Clone + fmt::Debug,
{
    // The resulting vector should contain the same amount of elements as
    // the slice that is being sorted, so enough room is preallocated
    let mut result: Vec<T> = Vec::with_capacity(arr.len());

    // Iterate over the elements to sort, the current element to insert is elem.
    for elem in arr.iter() {
        // How many elements have already been inserted?
        let n_inserted = result.len();

        // Copy or Clone the current element:
        let new_elem = elem.to_owned();

        // Loop over the inserted elements and one more index.
        for i in 0..n_inserted + 1 {
            if i == n_inserted || // Last inserted element
               &result[i] > elem
            {
                // Insert the element at i,
                // move the rest to higher indexes.
                result.insert(i, new_elem);
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn insertion() {
        use sorting::insertion::*;

        let res = insertion_sort(&Vec::<u8>::new());
        assert_eq!(res, vec![]);

        let res = insertion_sort(&vec!["a"]);
        assert_eq!(res, vec!["a"]);

        let res = insertion_sort(&vec!["a", "b", "c"]);
        assert_eq!(res, vec!["a", "b", "c"]);

        let res = insertion_sort(&vec!["d", "a", "c", "b"]);
        assert_eq!(res, vec!["a", "b", "c", "d"]);
    }
}
