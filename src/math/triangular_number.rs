// Triangular Numbers: Function to the Nth Triangular Number
// Wikipedia Reference : https://en.wikipedia.org/wiki/Triangular_number

// This program provides a function to calculate the nth triangular number defined by T_n = 1 + 2 +
// ... + n = (n^2 + 2)/2 = n(n + 1)/2 = (n + 1) choose 2.

//returns the nth triangular number
pub fn triangular_number(n: u64) -> u64 {
    (n | 1) * ((n + 1) / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_triangular_number {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (n, expected) =$inputs;
                assert_eq!(triangular_number(n), expected);
            }
        )*
        }
    }

    test_triangular_number! {
        input_0: (0, 0),
        input_1: (1, 1),
        input_2: (6, 21),
        input_3: (10, 55),
        input_4: (21, 231),
        input_5: (100, 5050),
    }
}
