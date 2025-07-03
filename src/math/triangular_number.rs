// Triangular Numbers: Function to the Nth Triangular Number
// Wikipedia Reference : https://en.wikipedia.org/wiki/Triangular_number

pub fn triangular_number(n: u64) -> u64 {
    let m: u64 = (n | 1) * ((n + 1) / 2);
    return m;
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
        input_0: (1, 1),
        input_1: (2, 3),
        input_2: (3, 6),
        input_3: (4, 10),
        input_4: (5, 15),
        input_5: (6, 21),
        input_6: (7, 28),
        input_7: (8, 36),
        input_8: (9, 45),
        input_9: (10, 55),
        input_10: (11, 66),
    }
}
