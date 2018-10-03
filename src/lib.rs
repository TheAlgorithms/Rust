pub mod searching;
pub mod sorting;

#[cfg(test)]
mod tests {
    use searching;

    #[test]
    fn linear() {
        use searching;
        let index = searching::linear_search(&"a", &vec!["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));

        let mut index = searching::linear_search(&4, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        index = searching::linear_search(&3, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        index = searching::linear_search(&2, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        index = searching::linear_search(&1, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(0));

        index = searching::linear_search(&5, &vec![1, 2, 3, 4]);
        assert_eq!(index, None);
    }
    #[test]
    fn binary() {
        use searching;
        let index = searching::binary_search(&"a", &vec!["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));

        let mut index = searching::binary_search(&4, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        index = searching::binary_search(&3, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        index = searching::binary_search(&2, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        index = searching::binary_search(&1, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(0));

        index = searching::binary_search(&5, &vec![1, 2, 3, 4]);
        assert_eq!(index, None);
    }

    use sorting;
    #[test]
    fn quick_sort() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        sorting::quick_sort(&mut ve1);
        for i in 0..ve1.len() - 1 {
            assert!(ve1[i] <= ve1[i + 1]);
        }

        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        sorting::quick_sort(&mut ve2);
        for i in 0..ve2.len() - 1 {
            assert!(ve2[i] <= ve2[i + 1]);
        }
    }
    #[test]
    fn bubble_sort() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        sorting::bubble_sort(&mut ve1);
        for i in 0..ve1.len() - 1 {
            assert!(ve1[i] <= ve1[i + 1]);
        }

        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        sorting::bubble_sort(&mut ve2);
        for i in 0..ve2.len() - 1 {
            assert!(ve2[i] <= ve2[i + 1]);
        }
    }
}
