/// Fibonacci via Dynamic Programming
use std::collections::HashMap;

/// fibonacci(n) returns the nth fibonacci number
/// This function uses the definition of Fibonacci where:
/// F(0) = F(1) = 1 and F(n+1) = F(n) + F(n-1) for n>0
///
/// Warning: This will overflow the 128-bit unsigned integer at n=186
pub fn fibonacci(n: u32) -> u128 {
    // Use a and b to store the previous two values in the sequence
    let mut a = 0;
    let mut b = 1;
    for _i in 0..n {
        // As we iterate through, move b's value into a and the new computed
        // value into b.
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

/// fibonacci(n) returns the nth fibonacci number
/// This function uses the definition of Fibonacci where:
/// F(0) = F(1) = 1 and F(n+1) = F(n) + F(n-1) for n>0
///
/// Warning: This will overflow the 128-bit unsigned integer at n=186
pub fn recursive_fibonacci(n: u32) -> u128 {
    // Call the actual tail recursive implementation, with the extra
    // arguments set up.
    _recursive_fibonacci(n, 0, 1)
}

fn _recursive_fibonacci(n: u32, previous: u128, current: u128) -> u128 {
    if n == 0 {
        current
    } else {
        _recursive_fibonacci(n - 1, current, current + previous)
    }
}

/// classical_fibonacci(n) returns the nth fibonacci number
/// This function uses the definition of Fibonacci where:
/// F(0) = 0, F(1) = 1 and F(n+1) = F(n) + F(n-1) for n>0
///
/// Warning: This will overflow the 128-bit unsigned integer at n=186
pub fn classical_fibonacci(n: u32) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let k = n / 2;
            let f1 = classical_fibonacci(k);
            let f2 = classical_fibonacci(k - 1);

            match n % 4 {
                0 | 2 => f1 * (f1 + 2 * f2),
                1 => (2 * f1 + f2) * (2 * f1 - f2) + 2,
                _ => (2 * f1 + f2) * (2 * f1 - f2) - 2,
            }
        }
    }
}

/// logarithmic_fibonacci(n) returns the nth fibonacci number
/// This function uses the definition of Fibonacci where:
/// F(0) = 0, F(1) = 1 and F(n+1) = F(n) + F(n-1) for n>0
///
/// Warning: This will overflow the 128-bit unsigned integer at n=186
pub fn logarithmic_fibonacci(n: u32) -> u128 {
    // if it is the max value before overflow, use n-1 then get the second
    // value in the tuple
    if n == 186 {
        let (_, second) = _logarithmic_fibonacci(185);
        second
    } else {
        let (first, _) = _logarithmic_fibonacci(n);
        first
    }
}

fn _logarithmic_fibonacci(n: u32) -> (u128, u128) {
    match n {
        0 => (0, 1),
        _ => {
            let (current, next) = _logarithmic_fibonacci(n / 2);
            let c = current * (next * 2 - current);
            let d = current * current + next * next;

            match n % 2 {
                0 => (c, d),
                _ => (d, c + d),
            }
        }
    }
}

/// Memoized fibonacci.
pub fn memoized_fibonacci(n: u32) -> u128 {
    let mut cache: HashMap<u32, u128> = HashMap::new();

    _memoized_fibonacci(n, &mut cache)
}

fn _memoized_fibonacci(n: u32, cache: &mut HashMap<u32, u128>) -> u128 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let f = match cache.get(&n) {
        Some(f) => f,
        None => {
            let f1 = _memoized_fibonacci(n - 1, cache);
            let f2 = _memoized_fibonacci(n - 2, cache);
            cache.insert(n, f1 + f2);
            cache.get(&n).unwrap()
        }
    };

    *f
}

/// matrix_fibonacci(n) returns the nth fibonacci number
/// This function uses the definition of Fibonacci where:
/// F(0) = 0, F(1) = 1 and F(n+1) = F(n) + F(n-1) for n>0
///
/// Matrix formula:
/// [F(n + 2)]  =  [1, 1] * [F(n + 1)]
/// [F(n + 1)]     [1, 0]   [F(n)    ]
///
/// Warning: This will overflow the 128-bit unsigned integer at n=186
pub fn matrix_fibonacci(n: u32) -> u128 {
    let multiplier: Vec<Vec<u128>> = vec![vec![1, 1], vec![1, 0]];

    let multiplier = matrix_power(&multiplier, n);
    let initial_fib_matrix: Vec<Vec<u128>> = vec![vec![1], vec![0]];

    let res = matrix_multiply(&multiplier, &initial_fib_matrix);

    res[1][0]
}

fn matrix_power(base: &Vec<Vec<u128>>, power: u32) -> Vec<Vec<u128>> {
    let identity_matrix: Vec<Vec<u128>> = vec![vec![1, 0], vec![0, 1]];

    vec![base; power as usize]
        .iter()
        .fold(identity_matrix, |acc, x| matrix_multiply(&acc, x))
}

// Copied from matrix_ops since u128 is required instead of i32
#[allow(clippy::needless_range_loop)]
fn matrix_multiply(multiplier: &[Vec<u128>], multiplicand: &[Vec<u128>]) -> Vec<Vec<u128>> {
    // Multiply two matching matrices. The multiplier needs to have the same amount
    // of columns as the multiplicand has rows.
    let mut result: Vec<Vec<u128>> = vec![];
    let mut temp;
    // Using variable to compare lenghts of rows in multiplicand later
    let row_right_length = multiplicand[0].len();
    for row_left in 0..multiplier.len() {
        if multiplier[row_left].len() != multiplicand.len() {
            panic!("Matrix dimensions do not match");
        }
        result.push(vec![]);
        for column_right in 0..multiplicand[0].len() {
            temp = 0;
            for row_right in 0..multiplicand.len() {
                if row_right_length != multiplicand[row_right].len() {
                    // If row is longer than a previous row cancel operation with error
                    panic!("Matrix dimensions do not match");
                }
                temp += multiplier[row_left][row_right] * multiplicand[row_right][column_right];
            }
            result[row_left].push(temp);
        }
    }
    result
}

/// nth_fibonacci_number_modulo_m(n, m) returns the nth fibonacci number modulo the specified m
/// i.e. F(n) % m
pub fn nth_fibonacci_number_modulo_m(n: i64, m: i64) -> i128 {
    let (length, pisano_sequence) = get_pisano_sequence_and_period(m);

    let remainder = n % length as i64;
    pisano_sequence.get(remainder as usize).unwrap().to_owned()
}

/// get_pisano_sequence_and_period(m) returns the Pisano Sequence and period for the specified integer m.
/// The pisano period is the period with which the sequence of Fibonacci numbers taken modulo m repeats.
/// The pisano sequence is the numbers in pisano period.
fn get_pisano_sequence_and_period(m: i64) -> (i128, Vec<i128>) {
    let mut a = 0;
    let mut b = 1;
    let mut lenght: i128 = 0;
    let mut pisano_sequence: Vec<i128> = vec![a, b];

    // Iterating through all the fib numbers to get the sequence
    for _i in 0..(m * m) + 1 {
        let c = (a + b) % m as i128;

        // adding number into the sequence
        pisano_sequence.push(c);

        a = b;
        b = c;

        if a == 0 && b == 1 {
            // Remove the last two elements from the sequence
            // This is a less elegant way to do it.
            pisano_sequence.pop();
            pisano_sequence.pop();
            lenght = pisano_sequence.len() as i128;
            break;
        }
    }

    (lenght, pisano_sequence)
}

/// last_digit_of_the_sum_of_nth_fibonacci_number(n) returns the last digit of the sum of n fibonacci numbers.
/// The function uses the definition of Fibonacci where:
/// F(0) = 0, F(1) = 1 and F(n+1) = F(n) + F(n-1) for n > 2
///
/// The sum of the Fibonacci numbers are:
/// F(0) + F(1) + F(2) + ... + F(n)
pub fn last_digit_of_the_sum_of_nth_fibonacci_number(n: i64) -> i64 {
    if n < 2 {
        return n;
    }

    // the pisano period of mod 10 is 60
    let n = ((n + 2) % 60) as usize;
    let mut fib = vec![0; n + 1];
    fib[0] = 0;
    fib[1] = 1;

    for i in 2..=n {
        fib[i] = (fib[i - 1] % 10 + fib[i - 2] % 10) % 10;
    }

    if fib[n] == 0 {
        return 9;
    }

    fib[n] % 10 - 1
}

#[cfg(test)]
mod tests {
    use super::classical_fibonacci;
    use super::fibonacci;
    use super::last_digit_of_the_sum_of_nth_fibonacci_number;
    use super::logarithmic_fibonacci;
    use super::matrix_fibonacci;
    use super::memoized_fibonacci;
    use super::nth_fibonacci_number_modulo_m;
    use super::recursive_fibonacci;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 1);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 2);
        assert_eq!(fibonacci(3), 3);
        assert_eq!(fibonacci(4), 5);
        assert_eq!(fibonacci(5), 8);
        assert_eq!(fibonacci(10), 89);
        assert_eq!(fibonacci(20), 10946);
        assert_eq!(fibonacci(100), 573147844013817084101);
        assert_eq!(fibonacci(184), 205697230343233228174223751303346572685);
    }

    #[test]
    fn test_recursive_fibonacci() {
        assert_eq!(recursive_fibonacci(0), 1);
        assert_eq!(recursive_fibonacci(1), 1);
        assert_eq!(recursive_fibonacci(2), 2);
        assert_eq!(recursive_fibonacci(3), 3);
        assert_eq!(recursive_fibonacci(4), 5);
        assert_eq!(recursive_fibonacci(5), 8);
        assert_eq!(recursive_fibonacci(10), 89);
        assert_eq!(recursive_fibonacci(20), 10946);
        assert_eq!(recursive_fibonacci(100), 573147844013817084101);
        assert_eq!(
            recursive_fibonacci(184),
            205697230343233228174223751303346572685
        );
    }

    #[test]
    fn test_classical_fibonacci() {
        assert_eq!(classical_fibonacci(0), 0);
        assert_eq!(classical_fibonacci(1), 1);
        assert_eq!(classical_fibonacci(2), 1);
        assert_eq!(classical_fibonacci(3), 2);
        assert_eq!(classical_fibonacci(4), 3);
        assert_eq!(classical_fibonacci(5), 5);
        assert_eq!(classical_fibonacci(10), 55);
        assert_eq!(classical_fibonacci(20), 6765);
        assert_eq!(classical_fibonacci(21), 10946);
        assert_eq!(classical_fibonacci(100), 354224848179261915075);
        assert_eq!(
            classical_fibonacci(184),
            127127879743834334146972278486287885163
        );
    }

    #[test]
    fn test_logarithmic_fibonacci() {
        assert_eq!(logarithmic_fibonacci(0), 0);
        assert_eq!(logarithmic_fibonacci(1), 1);
        assert_eq!(logarithmic_fibonacci(2), 1);
        assert_eq!(logarithmic_fibonacci(3), 2);
        assert_eq!(logarithmic_fibonacci(4), 3);
        assert_eq!(logarithmic_fibonacci(5), 5);
        assert_eq!(logarithmic_fibonacci(10), 55);
        assert_eq!(logarithmic_fibonacci(20), 6765);
        assert_eq!(logarithmic_fibonacci(21), 10946);
        assert_eq!(logarithmic_fibonacci(100), 354224848179261915075);
        assert_eq!(
            logarithmic_fibonacci(184),
            127127879743834334146972278486287885163
        );
    }

    #[test]
    /// Check that the itterative and recursive fibonacci
    /// produce the same value. Both are combinatorial ( F(0) = F(1) = 1 )
    fn test_iterative_and_recursive_equivalence() {
        assert_eq!(fibonacci(0), recursive_fibonacci(0));
        assert_eq!(fibonacci(1), recursive_fibonacci(1));
        assert_eq!(fibonacci(2), recursive_fibonacci(2));
        assert_eq!(fibonacci(3), recursive_fibonacci(3));
        assert_eq!(fibonacci(4), recursive_fibonacci(4));
        assert_eq!(fibonacci(5), recursive_fibonacci(5));
        assert_eq!(fibonacci(10), recursive_fibonacci(10));
        assert_eq!(fibonacci(20), recursive_fibonacci(20));
        assert_eq!(fibonacci(100), recursive_fibonacci(100));
        assert_eq!(fibonacci(184), recursive_fibonacci(184));
    }

    #[test]
    /// Check that classical and combinatorial fibonacci produce the
    /// same value when 'n' differs by 1.
    /// classical fibonacci: ( F(0) = 0, F(1) = 1 )
    /// combinatorial fibonacci: ( F(0) = F(1) = 1 )
    fn test_classical_and_combinatorial_are_off_by_one() {
        assert_eq!(classical_fibonacci(1), fibonacci(0));
        assert_eq!(classical_fibonacci(2), fibonacci(1));
        assert_eq!(classical_fibonacci(3), fibonacci(2));
        assert_eq!(classical_fibonacci(4), fibonacci(3));
        assert_eq!(classical_fibonacci(5), fibonacci(4));
        assert_eq!(classical_fibonacci(6), fibonacci(5));
        assert_eq!(classical_fibonacci(11), fibonacci(10));
        assert_eq!(classical_fibonacci(20), fibonacci(19));
        assert_eq!(classical_fibonacci(21), fibonacci(20));
        assert_eq!(classical_fibonacci(101), fibonacci(100));
        assert_eq!(classical_fibonacci(185), fibonacci(184));
    }

    #[test]
    fn test_memoized_fibonacci() {
        assert_eq!(memoized_fibonacci(0), 0);
        assert_eq!(memoized_fibonacci(1), 1);
        assert_eq!(memoized_fibonacci(2), 1);
        assert_eq!(memoized_fibonacci(3), 2);
        assert_eq!(memoized_fibonacci(4), 3);
        assert_eq!(memoized_fibonacci(5), 5);
        assert_eq!(memoized_fibonacci(10), 55);
        assert_eq!(memoized_fibonacci(20), 6765);
        assert_eq!(memoized_fibonacci(21), 10946);
        assert_eq!(memoized_fibonacci(100), 354224848179261915075);
        assert_eq!(
            memoized_fibonacci(184),
            127127879743834334146972278486287885163
        );
    }

    #[test]
    fn test_matrix_fibonacci() {
        assert_eq!(matrix_fibonacci(0), 0);
        assert_eq!(matrix_fibonacci(1), 1);
        assert_eq!(matrix_fibonacci(2), 1);
        assert_eq!(matrix_fibonacci(3), 2);
        assert_eq!(matrix_fibonacci(4), 3);
        assert_eq!(matrix_fibonacci(5), 5);
        assert_eq!(matrix_fibonacci(10), 55);
        assert_eq!(matrix_fibonacci(20), 6765);
        assert_eq!(matrix_fibonacci(21), 10946);
        assert_eq!(matrix_fibonacci(100), 354224848179261915075);
        assert_eq!(
            matrix_fibonacci(184),
            127127879743834334146972278486287885163
        );
    }

    #[test]
    fn test_nth_fibonacci_number_modulo_m() {
        assert_eq!(nth_fibonacci_number_modulo_m(5, 10), 5);
        assert_eq!(nth_fibonacci_number_modulo_m(10, 7), 6);
        assert_eq!(nth_fibonacci_number_modulo_m(20, 100), 65);
        assert_eq!(nth_fibonacci_number_modulo_m(1, 5), 1);
        assert_eq!(nth_fibonacci_number_modulo_m(0, 15), 0);
        assert_eq!(nth_fibonacci_number_modulo_m(50, 1000), 25);
        assert_eq!(nth_fibonacci_number_modulo_m(100, 37), 7);
        assert_eq!(nth_fibonacci_number_modulo_m(15, 2), 0);
        assert_eq!(nth_fibonacci_number_modulo_m(8, 1_000_000), 21);
        assert_eq!(nth_fibonacci_number_modulo_m(1000, 997), 996);
        assert_eq!(nth_fibonacci_number_modulo_m(200, 123), 0);
    }

    #[test]
    fn test_last_digit_of_the_sum_of_nth_fibonacci_number() {
        assert_eq!(last_digit_of_the_sum_of_nth_fibonacci_number(0), 0);
        assert_eq!(last_digit_of_the_sum_of_nth_fibonacci_number(1), 1);
        assert_eq!(last_digit_of_the_sum_of_nth_fibonacci_number(2), 2);
        assert_eq!(last_digit_of_the_sum_of_nth_fibonacci_number(3), 4);
        assert_eq!(last_digit_of_the_sum_of_nth_fibonacci_number(4), 7);
        assert_eq!(last_digit_of_the_sum_of_nth_fibonacci_number(5), 2);
        assert_eq!(last_digit_of_the_sum_of_nth_fibonacci_number(25), 7);
        assert_eq!(last_digit_of_the_sum_of_nth_fibonacci_number(50), 8);
        assert_eq!(last_digit_of_the_sum_of_nth_fibonacci_number(100), 5);
    }
}
