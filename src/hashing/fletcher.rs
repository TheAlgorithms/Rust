//! The Fletcher checksum is an algorithm for computing a position-dependent
//! checksum devised by John G. Fletcher (1934–2012) at Lawrence Livermore Labs
//! in the late 1970s. The objective of the Fletcher checksum was to provide
//! error-detection properties approaching those of a cyclic redundancy check
//! but with the lower computational effort associated with summation techniques.
//!
//! Reference: <https://en.wikipedia.org/wiki/Fletcher%27s_checksum>

/// Computes the Fletcher-16 checksum of an ASCII string.
///
/// Iterates over every byte in the input, maintaining two running sums
/// (`sum1` and `sum2`) each reduced modulo 255. The final 16-bit checksum
/// is produced by packing `sum2` into the high byte and `sum1` into the low byte.
///
/// # Arguments
///
/// * `data` - An ASCII string slice to checksum.
///
/// # Returns
///
/// A `u16` containing the Fletcher-16 checksum.
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::hashing::fletcher;
///
/// assert_eq!(fletcher("hello world"), 6752);
/// assert_eq!(fletcher("onethousandfourhundredthirtyfour"), 28347);
/// assert_eq!(fletcher("The quick brown fox jumps over the lazy dog."), 5655);
/// ```
pub fn fletcher(data: &str) -> u16 {
    let mut sum1: u16 = 0;
    let mut sum2: u16 = 0;

    for byte in data.bytes() {
        sum1 = (sum1 + byte as u16) % 255;
        sum2 = (sum2 + sum1) % 255;
    }

    (sum2 << 8) | sum1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        assert_eq!(fletcher("hello world"), 6752);
    }

    #[test]
    fn test_long_word() {
        assert_eq!(fletcher("onethousandfourhundredthirtyfour"), 28347);
    }

    #[test]
    fn test_pangram() {
        assert_eq!(
            fletcher("The quick brown fox jumps over the lazy dog."),
            5655
        );
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(fletcher(""), 0);
    }

    #[test]
    fn test_single_char() {
        // 'A' = 65; sum1 = 65 % 255 = 65, sum2 = 65 % 255 = 65
        // result = (65 << 8) | 65 = 16705
        assert_eq!(fletcher("A"), 16705);
    }
}
