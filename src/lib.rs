#[macro_use]
extern crate lazy_static;
pub mod backtracking;
pub mod big_integer;
pub mod ciphers;
pub mod data_structures;
pub mod dynamic_programming;
pub mod electronics;
pub mod general;
pub mod graph;
pub mod math;
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
        for i in 0..ve1.len() - 1 {
            assert!(ve1[i] <= ve1[i + 1]);
        }

        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        sorting::quick_sort(&mut ve2);
        for i in 0..ve2.len() - 1 {
            assert!(ve2[i] <= ve2[i + 1]);
        }
    }
}
