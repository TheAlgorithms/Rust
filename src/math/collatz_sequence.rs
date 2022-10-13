// collatz conjecture : https://en.wikipedia.org/wiki/Collatz_conjecture
pub fn sequence(mut n: usize) -> Option<Vec<usize>> {
    if n == 0 {
        return None;
    }
    let mut list: Vec<usize> = vec![];
    while n != 1 {
        list.push(n);
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }
    list.push(n);
    Some(list)
}

#[cfg(test)]
mod tests {
    use super::sequence;

    #[test]
    fn validity_check() {
        assert_eq!(sequence(10).unwrap(), [10, 5, 16, 8, 4, 2, 1]);
        assert_eq!(
            sequence(15).unwrap(),
            [15, 46, 23, 70, 35, 106, 53, 160, 80, 40, 20, 10, 5, 16, 8, 4, 2, 1]
        );
        assert_eq!(sequence(0).unwrap_or_else(|| vec![0]), [0]);
    }
}
