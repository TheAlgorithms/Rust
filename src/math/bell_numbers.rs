use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::sync::RwLock;

/// Returns the number of ways you can select r items given n options
fn n_choose_r(n: u32, r: u32) -> BigUint {
    if r == n || r == 0 {
        return One::one();
    }

    if r > n {
        return Zero::zero();
    }

    // Any combination will only need to be computed once, thus giving no need to
    // memoize this function

    let product: BigUint = (0..r).fold(BigUint::one(), |acc, x| {
        (acc * BigUint::from(n - x)) / BigUint::from(x + 1)
    });

    product
}

/// A memoization table for storing previous results
struct MemTable {
    buffer: Vec<BigUint>,
}

impl MemTable {
    const fn new() -> Self {
        MemTable { buffer: Vec::new() }
    }

    fn get(&self, n: usize) -> Option<BigUint> {
        if n == 0 || n == 1 {
            Some(BigUint::one())
        } else if let Some(entry) = self.buffer.get(n) {
            if *entry == BigUint::zero() {
                None
            } else {
                Some(entry.clone())
            }
        } else {
            None
        }
    }

    fn set(&mut self, n: usize, b: BigUint) {
        self.buffer[n] = b;
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    #[inline]
    fn resize(&mut self, new_size: usize) {
        if new_size > self.buffer.len() {
            self.buffer.resize(new_size, Zero::zero());
        }
    }
}

// Implemented with RwLock so it is accessible across threads
static LOOKUP_TABLE_LOCK: RwLock<MemTable> = RwLock::new(MemTable::new());

pub fn bell_number(n: u32) -> BigUint {
    let needs_resize;

    // Check if number is already in lookup table
    {
        let lookup_table = LOOKUP_TABLE_LOCK.read().unwrap();

        if let Some(entry) = lookup_table.get(n as usize) {
            return entry;
        }

        needs_resize = (n + 1) as usize > lookup_table.capacity();
    }

    // Resize table before recursion so that if more values need to be added during recursion the table isn't
    // reallocated every single time
    if needs_resize {
        let mut lookup_table = LOOKUP_TABLE_LOCK.write().unwrap();

        lookup_table.resize((n + 1) as usize);
    }

    let new_bell_number: BigUint = (0..n).map(|x| bell_number(x) * n_choose_r(n - 1, x)).sum();

    // Add new number to lookup table
    {
        let mut lookup_table = LOOKUP_TABLE_LOCK.write().unwrap();

        lookup_table.set(n as usize, new_bell_number.clone());
    }

    new_bell_number
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_choose_zero() {
        for i in 1..100 {
            assert_eq!(n_choose_r(i, 0), One::one());
        }
    }

    #[test]
    fn test_combination() {
        let five_choose_1 = BigUint::from(5u32);
        assert_eq!(n_choose_r(5, 1), five_choose_1);
        assert_eq!(n_choose_r(5, 4), five_choose_1);

        let ten_choose_3 = BigUint::from(120u32);
        assert_eq!(n_choose_r(10, 3), ten_choose_3);
        assert_eq!(n_choose_r(10, 7), ten_choose_3);

        let fourty_two_choose_thirty = BigUint::from_str("11058116888").unwrap();
        assert_eq!(n_choose_r(42, 30), fourty_two_choose_thirty);
        assert_eq!(n_choose_r(42, 12), fourty_two_choose_thirty);
    }

    #[test]
    fn test_bell_numbers() {
        let bell_one = BigUint::from(1u32);
        assert_eq!(bell_number(1), bell_one);

        let bell_three = BigUint::from(5u32);
        assert_eq!(bell_number(3), bell_three);

        let bell_eight = BigUint::from(4140u32);
        assert_eq!(bell_number(8), bell_eight);

        let bell_six = BigUint::from(203u32);
        assert_eq!(bell_number(6), bell_six);

        let bell_twenty_six = BigUint::from_str("49631246523618756274").unwrap();
        assert_eq!(bell_number(26), bell_twenty_six);
    }
}
