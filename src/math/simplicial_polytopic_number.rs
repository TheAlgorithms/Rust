use crate::math::factorial;
use crate::math::rising_factorial;

use std::ops::Div;

// Simplicial polytopic numbers are a family of sequences of figurate
// numbers corresponding to the d-dimensional simplex for each
// dimensional simplex for each dimension d, where d is a nonnegative
// integer
//
// For further reading:
// https://oeis.org/wiki/Simplicial_polytopic_numbers
// https://en.wikipedia.org/wiki/Triangular_number
//
// This program returns the simplicial polytopic number for any
// d-dimensional simplex. For example (2, 2) would return the
// second triangular number 3.
pub fn simplicial_polytopic_number(n: u64, d: u64) -> u64 {
    rising_factorial(n, d).div(factorial(d))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_simplicial_polytopic_number {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let ((n, d), expected) =$inputs;
                assert_eq!(simplicial_polytopic_number(n, d), expected);
            }
        )*
        }
    }

    test_simplicial_polytopic_number! {
        input_0: ((1, 1), 1),
        input_1: ((2, 1), 2),
        input_2: ((2, 2), 3),
        input_3: ((3, 3), 10),
        input_4: ((2, 4), 5),
        input_5: ((5, 5), 126),
        input_6: ((9, 6), 3003),
        input_7: ((6, 10), 3003),
    }
}
