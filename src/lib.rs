#[macro_use]
extern crate lazy_static;
pub mod backtracking;
pub mod big_integer;
pub mod bit_manipulation;
pub mod ciphers;
pub mod compression;
pub mod conversions;
pub mod data_structures;
pub mod dynamic_programming;
pub mod general;
pub mod geometry;
pub mod graph;
pub mod machine_learning;
pub mod math;
pub mod navigation;
pub mod number_theory;
pub mod searching;
pub mod sorting;
pub mod string;

#[cfg(test)]
mod tests {
    use super::sorting;
    #[test]
    fn quick_sort() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        sorting::quick_sort(&mut ve1);

        assert!(sorting::is_sorted(&ve1));

        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        sorting::quick_sort(&mut ve2);

        assert!(sorting::is_sorted(&ve2));
    }
}
