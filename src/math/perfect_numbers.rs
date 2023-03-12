pub fn is_perfect_number(num: usize) -> bool {
    let mut sum = 0;

    for i in 1..num - 1 {
        if num % i == 0 {
            sum += i;
        }
    }

    num == sum
}

pub fn perfect_numbers(max: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();

    // It is not known if there are any odd perfect numbers, so we go around all the numbers.
    for i in 1..max + 1 {
        if is_perfect_number(i) {
            result.push(i);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert!(is_perfect_number(6));
        assert!(is_perfect_number(28));
        assert!(is_perfect_number(496));
        assert!(is_perfect_number(8128));

        assert!(!is_perfect_number(5));
        assert!(!is_perfect_number(86));
        assert!(!is_perfect_number(497));
        assert!(!is_perfect_number(8120));

        assert_eq!(perfect_numbers(10), vec![6]);
        assert_eq!(perfect_numbers(100), vec![6, 28]);
        assert_eq!(perfect_numbers(496), vec![6, 28, 496]);
        assert_eq!(perfect_numbers(1000), vec![6, 28, 496]);
    }
}
