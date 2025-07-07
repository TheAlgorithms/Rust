// Tetrahedral Numbers: Function to the Nth Tetrahedral Number
// Wikipedia Reference : https://en.wikipedia.org/wiki/Tetrahedral_number
//
// This program provides a function to calculate the nth triangular
// number defined by T_n = 1 + 2 + ... + n = (n(n + 1)(n + 2))/6 =
// (n + 2) choose 3.

pub fn tetrahedral_number(n: u64) -> u64 {
    ((n) * (n + 1) * (n + 2)) / 6
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_tetrahedral_number {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (n, expected) =$inputs;
                assert_eq!(tetrahedral_number(n), expected);
            }
        )*
        }
    }

    test_tetrahedral_number! {
        input_0: (0, 0),
        input_1: (1, 1),
        input_2: (6, 56),
        input_3: (8, 120),
        input_4: (11, 286),
        input_5: (34, 7140),
    }
}
