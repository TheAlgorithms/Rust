#[doc = r"https://en.wikipedia.org/wiki/Diophantine_equation

linear diophantine equation
ax + by = c

"]

// Diophantine Equation : Given integers a,b,c ( at least one of a and b != 0), the
// diophantine equation a*x + b*y = c has a solution (where x and y are integers)
// iff gcd(a,b) divides c.

// GCD ( Greatest Common Divisor ) or HCF ( Highest Common Factor )

// >>> diophantine(10,6,14)
// (-7.0, 14.0)

// >>> diophantine(391,299,-69)
// (9.0, -12.0)

// But above equation has one more solution i.e., x = -4, y = 5.
// That's why we need diophantine all solution function.

// """

// """
// Lemma : if n|ab and gcd(a,n) = 1, then n|b.

// Finding All solutions of Diophantine Equations:

// Theorem : Let gcd(a,b) = d, a = d*p, b = d*q. If (x0,y0) is a solution of
// Diophantine Equation a*x + b*y = c.  a*x0 + b*y0 = c, then all the
// solutions have the form a(x0 + t*q) + b(y0 - t*p) = c,
// where t is an arbitrary integer.

// n is the number of solution you want, n = 2 by default

// >>> diophantine_all_soln(10, 6, 14,None)
// -7.0 14.0
// -4.0 9.0

// >>> diophantine_all_soln(10, 6, 14, Some(4))
// -7.0 14.0
// -4.0 9.0
// -1.0 4.0
// 2.0 -1.0

// >>> diophantine_all_soln(391, 299, -69, Some(4))
// 9.0 -12.0
// 22.0 -29.0
// 35.0 -46.0
// 48.0 -63.0

// """
// Euclid's Lemma :  d divides a and b, if and only if d divides a-b and b

// Euclid's Algorithm

// >>> greatest_common_divisor(7,5)
// 1

// Note : In number theory, two integers a and b are said to be relatively prime,
//        mutually prime, or co-prime if the only positive integer (factor) that
//        divides both of them is 1  i.e., gcd(a,b) = 1.

// >>> greatest_common_divisor(121, 11)
// 11

// """
// Extended Euclid's Algorithm : If d divides a and b and d = a*x + b*y for integers
// x and y, then d = gcd(a,b)

// >>> extended_gcd(10, 6)
// (2, -1, 2)

// >>> extended_gcd(7, 5)
// (1, -2, 3)
#[allow(dead_code)]
fn gcd(mut n: i64, mut m: i64) -> i64 {
    assert!(n > 0 && m > 0);
    if m < n {
        std::mem::swap(&mut m, &mut n);
    }
    while m % n != 0 {
        let t = m;
        let z = n;
        m = n;
        n = t % z;
    }
    n
}
#[allow(dead_code)]
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    assert!(a >= 0 && b >= 0);
    let (d, x, y, p, q): (i64, i64, i64, i64, i64);
    if b == 0 {
        (d, x, y) = (a, 1, 0);
    } else {
        (d, p, q) = extended_gcd(b, a % b);
        x = q;
        y = p - q * (((a / b) as f64).floor() as i64);
    }
    assert!(a % d == 0 && b % d == 0);
    assert!(d == a * x + b * y);
    (d, x, y)

}
#[allow(dead_code)]
fn diophantine(a: i64, b: i64, c: i64) -> (i64, i64) {
    assert!(c % gcd(a, b) == 0);

    let (d, x, y) = extended_gcd(a, b);
    let r = c / d;
    (r * x, r * y)
}
#[allow(dead_code)]
fn diophantine_all_soln(a: i64, b: i64, c: i64, mut n: Option<i64>) -> Vec<(i64, i64)> {
    let mut res: Vec<(i64, i64)> = vec![];
    let (x0, y0) = diophantine(a, b, c);
    let d = gcd(a, b);
    let p = ((a / d) as f64).floor() as i64;
    let q = ((b / d) as f64).floor() as i64;
    let (mut x, mut y): (i64, i64);
    if n.is_none() {
        n = Some(2)
    }
    for i in 0..n.unwrap() {
        x = x0 + i * q;
        y = y0 - i * p;
        res.push((x, y));
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_diophantine() {
        assert!(diophantine(10, 6, 14) == (-7, 14));
    }
    #[test]
    fn test_diophantine_all_soln() {
        assert!(diophantine_all_soln(10, 6, 14, None) == vec![(-7, 14), (-4, 9)])
    }
}
