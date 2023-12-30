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

    #[test]
    fn test_recursive_lucas_number() {
        assert_eq!(recursive_lucas_number(1), 1);
        assert_eq!(recursive_lucas_number(20), 15127);
        assert_eq!(recursive_lucas_number(0), 2);
        assert_eq!(recursive_lucas_number(25), 167761);
    }

    #[test]
    fn test_dynamic_lucas_number() {
        assert_eq!(dynamic_lucas_number(1), 1);
        assert_eq!(dynamic_lucas_number(20), 15127);
        assert_eq!(dynamic_lucas_number(0), 2);
        assert_eq!(dynamic_lucas_number(25), 167761);
    }
}
