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
        (a, b) = (b, a + b);
    }

    a
}

pub fn dynamic_lucas_number_logn(n: u32) -> u32 {
    if n == 0 {
        return 2;
    } else if n == 1 {
        return 1;
    }

    // Matrix exponentiation: [[1, 1], [1, 0]]^n
    // say n = 11
    // We can write 11 as 1011 in binary
    // which is 2^3 * 1 + 2^1 * 1 + 2^0 * 1
    let mut matrix = [[1u32, 1u32], [1u32, 0u32]];
    let mut result = [[1u32, 0u32], [0u32, 1u32]]; // Identity matrix
    let mut power = n - 1;

    while power > 0 {
        if power & 1 == 1 {
            result = matrix_multiply(result, matrix);
        }
        // always square the matrix, this will generate
        // the following sequence for each iteration of
        // i: matrix^(2^i) for i = 1, 2, 3...
        matrix = matrix_multiply(matrix, matrix);
        power >>= 1;
    }

    // Return L(n) = result[0][0] * L(1) + result[0][1] * L(0) = result[0][0] * 1 + result[0][1] * 2
    result[0][0] + 2 * result[0][1]
}

fn matrix_multiply(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2] {
    let mut result = [[0u32; 2]; 2];

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                result[i][j] = result[i][j].wrapping_add(a[i][k].wrapping_mul(b[k][j]));
            }
        }
    }

    result
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
                assert_eq!(dynamic_lucas_number_logn(n), expected);
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
