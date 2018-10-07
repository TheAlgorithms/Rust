pub fn radix_sort(data: &mut [isize]) {
    // radix is 256 for faster sort
    let mut buckets = vec![vec![]; 256];

    for n in 0..4 {
        for r in data as &[isize] {
            let val = *r;
            // get the nth byte which has 256 possible values
            let mut which = (val >> (8 * n)) & 0xFFisize; // It's a bit like (val / (256 to the power of n)) % 256
            if n == 3 {
                //  The XOR instruction (^) is to flip the sign bit so that negative values work properly
                which ^= 0x80;
            }
            buckets[which as usize].push(val);
        }

        let mut i = 0;
        for b in buckets.iter_mut() {
            for r in b as &[isize] {
                data[i] = *r;
                i += 1;
            }
            b.clear();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn shell_sort_test() {
        // test for reverse array
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        radix_sort(&mut ve1);
        for i in 0..ve1.len() - 1 {
            assert!(ve1[i] <= ve1[i + 1]);
        }

        // test for already sorted array
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        radix_sort(&mut ve2);
        for i in 0..ve2.len() - 1 {
            assert!(ve2[i] <= ve2[i + 1]);
        }

        // test for unsorted
        let mut ve3 = vec![3, 5, 6, 3, 1, 4];
        radix_sort(&mut ve3);
        for i in 0..ve3.len() - 1 {
            assert!(ve3[i] <= ve3[i + 1]);
        }
    }
}
