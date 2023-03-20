use num_traits::CheckedAdd;

pub fn binary_to_decimal(binary: &str) -> Option<u128> {
    if binary.len() > 128 {
        return None;
    }
    let mut num = 0;
    let mut idx_val = 1;
    for bit in binary.chars().rev() {
        match bit {
            '1' => {
                if let Some(sum) = num.checked_add(&idx_val) {
                    num = sum;
                } else {
                    return None;
                }
            }
            '0' => {}
            _ => return None,
        }
        idx_val <<= 1;
    }
    Some(num)
}

#[cfg(test)]
mod tests {
    use super::binary_to_decimal;

    #[test]
    fn basic_binary_to_decimal() {
        assert_eq!(binary_to_decimal("0000000110"), Some(6));
        assert_eq!(binary_to_decimal("1000011110"), Some(542));
        assert_eq!(binary_to_decimal("1111111111"), Some(1023));
    }
    #[test]
    fn big_binary_to_decimal() {
        assert_eq!(
            binary_to_decimal("111111111111111111111111"),
            Some(16_777_215)
        );
        // 32 bits
        assert_eq!(
            binary_to_decimal("11111111111111111111111111111111"),
            Some(4_294_967_295)
        );
        // 64 bits
        assert_eq!(
            binary_to_decimal("1111111111111111111111111111111111111111111111111111111111111111"),
            Some(18_446_744_073_709_551_615u128)
        );
    }
    #[test]
    fn very_big_binary_to_decimal() {
        // 96 bits
        assert_eq!(
            binary_to_decimal(
                "1111111111111111111111111111111111111111111111111111111111111111\
                11111111111111111111111111111111"
            ),
            Some(79_228_162_514_264_337_593_543_950_335u128)
        );

        // 128 bits
        assert_eq!(
            binary_to_decimal(
                "1111111111111111111111111111111111111111111111111111111111111111\
                1111111111111111111111111111111111111111111111111111111111111111"
            ),
            Some(340_282_366_920_938_463_463_374_607_431_768_211_455u128)
        );
        // 129 bits, should overflow
        assert!(binary_to_decimal(
            "1111111111111111111111111111111111111111111111111111111111111111\
                11111111111111111111111111111111111111111111111111111111111111111"
        )
        .is_none());
        // obviously none
        assert!(binary_to_decimal(
            "1111111111111111111111111111111111111111111111111111111111111111\
                1111111111111111111111111111111111111111111111111111111111111\
                1111111111111111111111111111111111111111111111111111111111111\
                1111111111111111111111111111111111111111111111111111111111111\
                1111111111111111111111111111111111111111111111111111111111111\
                1111111111111111111111111111111111111111111111111111111111111\
                1111111111111111111111111111111111111111111111111111111111111"
        )
        .is_none());
    }
}
