/// Swaps odd and even bits in an integer.
///
/// This function separates the even bits (0, 2, 4, 6, etc.) and odd bits (1, 3, 5, 7, etc.)
/// using bitwise AND operations, then swaps them by shifting and combining with OR.
///
/// # Arguments
///
/// * `num` - A 32-bit unsigned integer
///
/// # Returns
///
/// A new integer with odd and even bits swapped
///
/// # Examples
///
/// ```
/// use rust_algorithms::bit_manipulation::swap_odd_even_bits;
///
/// assert_eq!(swap_odd_even_bits(0), 0);
/// assert_eq!(swap_odd_even_bits(1), 2);
/// assert_eq!(swap_odd_even_bits(2), 1);
/// assert_eq!(swap_odd_even_bits(3), 3);
/// assert_eq!(swap_odd_even_bits(4), 8);
/// assert_eq!(swap_odd_even_bits(5), 10);
/// assert_eq!(swap_odd_even_bits(6), 9);
/// assert_eq!(swap_odd_even_bits(23), 43);
/// ```
pub fn swap_odd_even_bits(num: u32) -> u32 {
    // Get all even bits - 0xAAAAAAAA is a 32-bit number with all even bits set to 1
    let even_bits = num & 0xAAAAAAAA;
    
    // Get all odd bits - 0x55555555 is a 32-bit number with all odd bits set to 1
    let odd_bits = num & 0x55555555;
    
    // Right shift even bits and left shift odd bits and swap them
    (even_bits >> 1) | (odd_bits << 1)
}

/// Formats binary representation of two numbers for display.
///
/// # Arguments
///
/// * `before` - The original number
/// * `after` - The transformed number
///
/// # Returns
///
/// A formatted string showing both numbers in binary
fn show_bits(before: u32, after: u32) -> String {
    format!("{:>5}: {:08b}\n{:>5}: {:08b}", before, before, after, after)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_odd_even_bits() {
        assert_eq!(swap_odd_even_bits(0), 0);
        assert_eq!(swap_odd_even_bits(1), 2);
        assert_eq!(swap_odd_even_bits(2), 1);
        assert_eq!(swap_odd_even_bits(3), 3);
        assert_eq!(swap_odd_even_bits(4), 8);
        assert_eq!(swap_odd_even_bits(5), 10);
        assert_eq!(swap_odd_even_bits(6), 9);
        assert_eq!(swap_odd_even_bits(23), 43);
    }

    #[test]
    fn test_show_bits() {
        assert_eq!(
            show_bits(0, swap_odd_even_bits(0)),
            "    0: 00000000\n    0: 00000000"
        );
        assert_eq!(
            show_bits(1, swap_odd_even_bits(1)),
            "    1: 00000001\n    2: 00000010"
        );
        assert_eq!(
            show_bits(23, swap_odd_even_bits(23)),
            "   23: 00010111\n   43: 00101011"
        );
    }

    #[test]
    fn test_display_output() {
        let test_cases = vec![0, 1, 2, 3, 4, 23, 24];
        
        for i in test_cases {
            let result = show_bits(i, swap_odd_even_bits(i));
            println!("{}\n", result);
        }
    }
}

fn main() {
    println!("Swapping odd and even bits:\n");
    
    for i in [0, 1, 2, 3, 4, 23, 24] {
        println!("{}\n", show_bits(i, swap_odd_even_bits(i)));
    }
}
