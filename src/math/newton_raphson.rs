pub fn find_root(f: fn(f64) -> f64, fd: fn(f64) -> f64, guess: f64, iterations: i32) -> f64 {
    let mut result = guess;
    for _ in 0..iterations {
        result = iteration(f, fd, result);
    }
    result
}

pub fn iteration(f: fn(f64) -> f64, fd: fn(f64) -> f64, guess: f64) -> f64 {
    guess - f(guess) / fd(guess)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn math_fn(x: f64) -> f64 {
        x.cos() - (x * x * x)
    }
    fn math_fnd(x: f64) -> f64 {
        -x.sin() - 3.0 * (x * x)
    }
    #[test]
    fn basic() {
        assert_eq!(find_root(math_fn, math_fnd, 0.5, 6), 0.8654740331016144);
    }
}
