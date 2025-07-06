// Tetrahedral Numbers: Function to the Nth Tetrahedral Number
// Wikipedia Reference : https://en.wikipedia.org/wiki/Tetrahedral_number
use num_bigint::BigUint;
pub fn tetrahedral_number(n: u64) -> BigUint {
    let m: BigUint = (((n) * (n + 1) * (n + 2)) / 6).into();
    m
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
        input_6: (6, BigUint::from(56u32)),
        input_8: (8, BigUint::from(120u32)),
        input_34: (34, BigUint::from(7140u32)),
    }
}
