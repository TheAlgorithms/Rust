// Author : cyrixninja
// Lucas Series : Function to get the Nth Lucas Number
// Wikipedia Reference  :  https://en.wikipedia.org/wiki/Lucas_number
// Other References     :  https://the-algorithms.com/algorithm/lucas-series?lang=python

pub fn recursive_lucas_number(n: u32) -> u32 {
    match n {
        0 => 2,
        1 => 1,
        _ => recursive_lucas_number(n - 1) + recursive_lucas_number(n - 2),
    }
}

pub fn dynamic_lucas_number(n: u32) -> u32 {
    let mut a = 2;
    let mut b = 1;

    for _ in 0..n {
        let temp = a;
        a = b;
        b += temp;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_lucas_number {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (n, expected) = $inputs;
                assert_eq!(recursive_lucas_number(n), expected);
                assert_eq!(dynamic_lucas_number(n), expected);
            }
        )*
        }
    }

    test_lucas_number! {
        input_0: (0, 2),
        input_1: (1, 1),
        input_2: (2, 3),
        input_3: (3, 4),
        input_4: (4, 7),
        input_5: (5, 11),
        input_6: (6, 18),
        input_7: (7, 29),
        input_8: (8, 47),
        input_9: (9, 76),
        input_10: (10, 123),
        input_15: (15, 1364),
        input_20: (20, 15127),
        input_25: (25, 167761),
    }
}
