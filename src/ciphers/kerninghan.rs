pub fn kerninghan(n: u32) -> i32 {
    let mut count = 0;
    let mut n = n;

    while n > 0 {
        n = n & (n - 1);
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_set_bits() {
        assert_eq!(kerninghan(0b_00000000000000000000000000001011), 3);
        assert_eq!(kerninghan(0b_00000000000000000000000010000000), 1);
        assert_eq!(kerninghan(0b_11111111111111111111111111111101), 31);
    }
}
