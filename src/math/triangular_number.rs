// Triangular Numbers: Function to the Nth Triangular Number
// Wikipedia Reference : https://en.wikipedia.org/wiki/Triangular_number

pub fn triangular_number(n: u64) -> u64 {
    let m: u64 = (n | 1) * ((n + 1) / 2);
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
        input_5: (6, 21),
        input_9: (10, 55),
        input_10: (100, 5050),
    }
}
