pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        let mut smallest = i;
        for j in (i + 1)..len {
            if arr[j] < arr[smallest] {
                smallest = j;
            }
        }
        arr.swap(smallest, i);
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
