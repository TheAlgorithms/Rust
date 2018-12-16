pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for left in 0..len {
        let mut smallest = left;
        for right in (left + 1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(smallest, left);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sort_numbers() {
        use sorting::selection_sort;

        let mut res = Vec::<u8>::new();
        selection_sort(&mut res);
        assert_eq!(res, vec![]);

        let mut res = vec!["a"];
        selection_sort(&mut res);
        assert_eq!(res, vec!["a"]);

        let mut res = vec!["a", "b", "c"];
        selection_sort(&mut res);
        assert_eq!(res, vec!["a", "b", "c"]);

        let mut res = vec!["d", "a", "c", "b"];
        selection_sort(&mut res);
        assert_eq!(res, vec!["a", "b", "c", "d"]);
    }
}
