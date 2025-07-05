// Triangular Numbers: Function to the Nth Triangular Number
// Wikipedia Reference : https://en.wikipedia.org/wiki/Triangular_number
use num_bigint::BigUint;
pub fn triangular_number(n: u64) -> BigUint {
    let m: BigUint = ((n | 1) * ((n + 1) / 2)).into();
    m
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
        input_5: (6, BigUint::from(21u32)),
        input_9: (10, BigUint::from(55u32)),
        input_10: (100, BigUint::from(5050u32)),
    }
}
