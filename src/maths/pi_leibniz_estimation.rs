pub fn estimate_pi_leibniz(n: u32) -> f64 {
    // π = (4/1) - (4/3) + (4/5) - (4/7) + (4/9) - (4/11) + (4/13) - (4/15) ...
    4.0 * (1..=n).fold(1.0, |acc, i| {
        acc + (if i % 2 == 1 { -1.0 } else { 1.0 }) / (2.0 * i as f64 + 1.0)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        let epsilon = 0.0001;
        let s = estimate_pi_leibniz(10000);
        assert!((s - std::f64::consts::PI) <= epsilon);
    }
}
