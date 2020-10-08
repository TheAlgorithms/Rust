pub fn radix_sort(input: &mut [u32]) {
    if input.len() == 0 {
        return;
    }

    // initialize 10 buckets
    let mut buckets: Vec<Vec<u32>> = Vec::with_capacity(10);
    for _ in 0..10 {
        buckets.push(Vec::new());
    }

    // Getting max digits for number
    let count: usize = input.iter().max().unwrap().to_string().len();

    let mut divisor: u32 = 1;

    for _ in 0..count {
        for num in input.iter() {
            let temp = (num / divisor) as usize;
            buckets[temp % 10].push(*num);
        }

        let mut j: usize = 0;

        for i in 0..10 {
            for bucket in buckets[i].iter() {
                input[j] = *bucket;
                j += 1;
            }
            buckets[i].clear();
        }
        divisor *= 10;
    }
}

#[cfg(test)]
mod test {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn radix_sort_descending() {
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        radix_sort(&mut ve1);

        assert!(is_sorted(&ve1));
    }

    #[test]
    fn radix_sort_pre_sorted() {
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        radix_sort(&mut ve2);

        assert!(is_sorted(&ve2));
    }
}
