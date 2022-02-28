mod extended_euclidean_algorithm;
mod greatest_common_divisor;
mod pascal_triangle;
mod perfect_numbers;
mod prime_check;
mod prime_numbers;
mod trial_division;
mod square_root;

pub use self::extended_euclidean_algorithm::extended_euclidean_algorithm;
pub use self::greatest_common_divisor::{
    greatest_common_divisor_iterative, greatest_common_divisor_recursive,
};
pub use self::pascal_triangle::pascal_triangle;
pub use self::perfect_numbers::perfect_numbers;
pub use self::prime_check::prime_check;
pub use self::prime_numbers::prime_numbers;
pub use self::trial_division::trial_division;
pub use self::square_root::square_root;
