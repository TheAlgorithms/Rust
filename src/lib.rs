mod searching;
mod sorting;

#[cfg(test)]
mod tests {
    use searching;

    #[test]
    fn binary_search() {
        let mut index = searching::binary_search(4, &vec![1, 2, 3, 4]);
        assert_eq!(index, 3);

        index = searching::binary_search(3, &vec![1, 2, 3, 4]);
        assert_eq!(index, 2);

        index = searching::binary_search(2, &vec![1, 2, 3, 4]);
        assert_eq!(index, 1);

        index = searching::binary_search(1, &vec![1, 2, 3, 4]);
        assert_eq!(index, 0);

        index = searching::binary_search(5, &vec![1, 2, 3, 4]);
        assert_eq!(index, -1);

        index = searching::binary_search("a", &vec!["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, 0);

        index = searching::binary_search("google", &vec!["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, 4);
    }
}
