mod binary_coded_decimal;
mod binary_count_trailing_zeros;
mod binary_shifts;
mod counting_bits;
mod find_missing_number;
mod find_previous_power_of_two;
mod find_unique_number;
mod hamming_distance;
mod highest_set_bit;
mod is_power_of_two;
mod n_bits_gray_code;
mod reverse_bits;
mod rightmost_set_bit;
mod sum_of_two_integers;
mod swap_odd_even_bits;
mod twos_complement;

pub use self::binary_coded_decimal::binary_coded_decimal;
pub use self::binary_count_trailing_zeros::binary_count_trailing_zeros;
pub use self::binary_shifts::{
    arithmetic_left_shift, arithmetic_right_shift, logical_left_shift, logical_right_shift,
};
pub use self::counting_bits::count_set_bits;
pub use self::find_missing_number::find_missing_number;
pub use self::find_previous_power_of_two::find_previous_power_of_two;
pub use self::find_unique_number::find_unique_number;
pub use self::hamming_distance::{hamming_distance, hamming_distance_str};
pub use self::highest_set_bit::find_highest_set_bit;
pub use self::is_power_of_two::is_power_of_two;
pub use self::n_bits_gray_code::generate_gray_code;
pub use self::reverse_bits::reverse_bits;
pub use self::rightmost_set_bit::{index_of_rightmost_set_bit, index_of_rightmost_set_bit_log};
pub use self::sum_of_two_integers::add_two_integers;
pub use self::swap_odd_even_bits::swap_odd_even_bits;
pub use self::twos_complement::twos_complement;
