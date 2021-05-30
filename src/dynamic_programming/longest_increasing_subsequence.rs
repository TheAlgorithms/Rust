pub fn longest_increasing_subsequence<'a>(a: &'a[i32], b: &[i32]) -> &'a[i32] {
    panic!("longest_increasing_subsequence not implemented yet!");
}

#[cfg(test)]
mod tests {
    use super::longest_increasing_subsequence;

    #[test]
    fn test_longest_increasing_subsequence() {
        assert_eq!(&longest_increasing_subsequence(&[], &[]), []);
    }
}