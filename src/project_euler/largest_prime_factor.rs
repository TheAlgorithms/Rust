
/// Get the largest prime factor of the (unsigned) integer `n`
fn largest_prime_factor(mut n: u64) -> u64 {
    // Largest prime factor
    let mut lpf = 2;
    while n > lpf {
        if n % lpf == 0 {
            n /= lpf;
            lpf = 2;
        } else {
            lpf += 1;
        }
    }

    return lpf;
}

#[test]
fn problem_test() {
    // The problem asks for largest prime factor of 600851475143
    assert_eq!(largest_prime_factor(600851475143), 6857);
}

#[test]
fn two() {
    assert_eq!(largest_prime_factor(2), 2);
}

#[test]
fn ten() {
    assert_eq!(largest_prime_factor(10), 5);
}
