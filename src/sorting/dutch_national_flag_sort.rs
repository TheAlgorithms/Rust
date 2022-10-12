/*
A Rust implementation of the Dutch National Flag sorting algorithm.

Reference implementation: https://github.com/TheAlgorithms/Python/blob/master/sorts/dutch_national_flag_sort.py
More info: https://en.wikipedia.org/wiki/Dutch_national_flag_problem
*/

#[derive(PartialOrd, PartialEq, Eq)]
pub enum Colors {
    Red,   // \
    White, //  | Define the three colors of the Dutch Flag: 🇳🇱
    Blue,  // /
}
use Colors::*;

// Algorithm implementation
pub fn dutch_national_flag_sort(mut sequence: Vec<Colors>) -> Vec<Colors> {
    // We take ownership of `sequence` because the original `sequence` will be modified and then returned
    let length = sequence.len();
    if length <= 1 {
        return sequence; // Arrays of length 0 or 1 are automatically sorted
    }
    let mut low = 0;
    let mut mid = 0;
    let mut high = length - 1;
    while mid <= high {
        match sequence[mid] {
            Red => {
                sequence.swap(low, mid);
                low += 1;
                mid += 1;
            }
            White => {
                mid += 1;
            }
            Blue => {
                sequence.swap(mid, high);
                high -= 1;
            }
        }
    }
    sequence
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn random_array() {
        let arr = vec![
            Red, Blue, White, White, Blue, Blue, Red, Red, White, Blue, White, Red, White, Blue,
        ];
        let arr = dutch_national_flag_sort(arr);
        assert!(is_sorted(&arr))
    }

    #[test]
    fn sorted_array() {
        let arr = vec![
            Red, Red, Red, Red, Red, White, White, White, White, White, Blue, Blue, Blue, Blue,
        ];
        let arr = dutch_national_flag_sort(arr);
        assert!(is_sorted(&arr))
    }
}
