pub fn faster_radix_sort(data: &mut [isize]) {
    // radix is 256 for faster sort
    let mut buckets = vec![vec![]; 256];

    for z in 0..4 {
        for r in data as &[isize] {
            let val = *r;
            // get the nth byte which has 256 possible values
            let mut which = (val >> (8 * z)) & 0xFFisize; // It's a bit like (val / (256 to the power of n)) % 256
            if z == 3 {
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

// Simple version of radix sort

// A simple function to get maximum value in data[]
fn get_max(data: &[isize]) -> isize {
    let mut max = data[0];

    for i in data {
        if *i > max {
            max = *i;
        }
    }
    max
}

// A function to do counting sort of data[] according to
// the digit represented by exp
fn count_sort(data: &mut [isize], exp: isize) {
    let n = data.len();

    let mut new_data = vec![0isize; n]; // to store sorted values

    // Store count of occurrences in count[]
    let mut count = vec![0isize; 10];

    for k in 0..n {
        count[(data[k] / exp % 10) as usize] += 1;
    }

    // Change count[i] so that count[i] now contains actual
    //  position of this digit in new_data[]
    for i in 1..10 {
        count[i] += count[i - 1];
    }

    // Build the new_data[]
    for z in (0..=n - 1).rev() {
        new_data[(count[((data[z] / exp) % 10) as usize] - 1) as usize] = data[z];

        count[((data[z] / exp) % 10) as usize] -= 1;
    }

    for t in 0..n {
        data[t] = new_data[t];
    }
}

pub fn radix_sort(data: &mut [isize]) {
    let max = get_max(&data);

    let mut exp = 1;

    // Do counting sort for every digit. Note that instead
    // of passing digit number, exp is passed. exp is 10^i
    // where i is current digit number
    while max / exp > 0 {
        count_sort(data, exp);
        exp *= 10;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn radix_sort_test() {
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

    #[test]
    fn faster_radix_sort_test() {
        // test for reverse array
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        faster_radix_sort(&mut ve1);
        for i in 0..ve1.len() - 1 {
            assert!(ve1[i] <= ve1[i + 1]);
        }

        // test for already sorted array
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        faster_radix_sort(&mut ve2);
        for i in 0..ve2.len() - 1 {
            assert!(ve2[i] <= ve2[i + 1]);
        }

        // test for unsorted
        let mut ve3 = vec![3, 5, 6, 3, 1, 4];
        faster_radix_sort(&mut ve3);
        for i in 0..ve3.len() - 1 {
            assert!(ve3[i] <= ve3[i + 1]);
        }
    }
}
