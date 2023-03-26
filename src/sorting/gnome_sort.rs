use std::cmp;

pub fn gnome_sort<T>(arr: &[T]) -> Vec<T>
where
    T: cmp::PartialEq + cmp::PartialOrd + Clone,
{
    let mut arr = arr.to_vec();
    let mut i: usize = 1;
    let mut j: usize = 2;

    while i < arr.len() {
        if arr[i - 1] < arr[i] {
            i = j;
            j = i + 1;
        } else {
            arr.swap(i - 1, i);
            i -= 1;
            if i == 0 {
                i = j;
                j += 1;
            }
        }
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let res = gnome_sort(&[6, 5, -8, 3, 2, 3]);
        assert!(crate::sorting::is_sorted(&res));
    }

    #[test]
    fn already_sorted() {
        let res = gnome_sort(&["a", "b", "c"]);
        assert!(crate::sorting::is_sorted(&res));
    }

    #[test]
    fn odd_number_of_elements() {
        let res = gnome_sort(&["d", "a", "c", "e", "b"]);
        assert!(crate::sorting::is_sorted(&res));
    }

    #[test]
    fn one_element() {
        let res = gnome_sort(&[3]);
        assert!(crate::sorting::is_sorted(&res));
    }

    #[test]
    fn empty() {
        let res = gnome_sort(&Vec::<u8>::new());
        assert!(crate::sorting::is_sorted(&res));
    }
}
