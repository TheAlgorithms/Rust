use std::ops::{Add, Mul, MulAssign, Sub};

// f64 complex
#[derive(Clone, Copy, Debug)]
pub struct Complex64 {
    pub re: f64,
    pub im: f64,
}

impl Complex64 {
    #[inline]
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    #[inline]
    pub fn square_norm(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    #[inline]
    pub fn norm(&self) -> f64 {
        self.square_norm().sqrt()
    }

    #[inline]
    pub fn inverse(&self) -> Complex64 {
        let nrm = self.square_norm();
        Complex64 {
            re: self.re / nrm,
            im: -self.im / nrm,
        }
    }
}

impl Default for Complex64 {
    #[inline]
    fn default() -> Self {
        Self { re: 0.0, im: 0.0 }
    }
}

impl Add<Complex64> for Complex64 {
    type Output = Complex64;

    #[inline]
    fn add(self, other: Complex64) -> Complex64 {
        Complex64 {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl Sub<Complex64> for Complex64 {
    type Output = Complex64;

    #[inline]
    fn sub(self, other: Complex64) -> Complex64 {
        Complex64 {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

impl Mul<Complex64> for Complex64 {
    type Output = Complex64;

    #[inline]
    fn mul(self, other: Complex64) -> Complex64 {
        Complex64 {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl MulAssign<Complex64> for Complex64 {
    #[inline]
    fn mul_assign(&mut self, other: Complex64) {
        let tmp = self.re * other.im + self.im * other.re;
        self.re = self.re * other.re - self.im * other.im;
        self.im = tmp;
    }
}

pub fn fast_fourier_transform_input_permutation(length: usize) -> Vec<usize> {
    let mut result = Vec::new();
    result.reserve_exact(length);
    for i in 0..length {
        result.push(i);
    }
    let mut reverse = 0_usize;
    let mut position = 1_usize;
    while position < length {
        let mut bit = length >> 1;
        while bit & reverse != 0 {
            reverse ^= bit;
            bit >>= 1;
        }
        reverse ^= bit;
        // This is equivalent to adding 1 to a reversed number
        if position < reverse {
            // Only swap each element once
            result.swap(position, reverse);
        }
        position += 1;
    }
    result
}

pub fn fast_fourier_transform(input: &[f64], input_permutation: &[usize]) -> Vec<Complex64> {
    let n = input.len();
    let mut result = Vec::new();
    result.reserve_exact(n);
    for position in input_permutation {
        result.push(Complex64::new(input[*position], 0.0));
    }
    let mut segment_length = 1_usize;
    while segment_length < n {
        segment_length <<= 1;
        let angle: f64 = std::f64::consts::TAU / segment_length as f64;
        let w_len = Complex64::new(angle.cos(), angle.sin());
        for segment_start in (0..n).step_by(segment_length) {
            let mut w = Complex64::new(1.0, 0.0);
            for position in segment_start..(segment_start + segment_length / 2) {
                let a = result[position];
                let b = result[position + segment_length / 2] * w;
                result[position] = a + b;
                result[position + segment_length / 2] = a - b;
                w *= w_len;
            }
        }
    }
    result
}

pub fn inverse_fast_fourier_transform(
    input: &[Complex64],
    input_permutation: &[usize],
) -> Vec<f64> {
    let n = input.len();
    let mut result = Vec::new();
    result.reserve_exact(n);
    for position in input_permutation {
        result.push(input[*position]);
    }
    let mut segment_length = 1_usize;
    while segment_length < n {
        segment_length <<= 1;
        let angle: f64 = -std::f64::consts::TAU / segment_length as f64;
        let w_len = Complex64::new(angle.cos(), angle.sin());
        for segment_start in (0..n).step_by(segment_length) {
            let mut w = Complex64::new(1.0, 0.0);
            for position in segment_start..(segment_start + segment_length / 2) {
                let a = result[position];
                let b = result[position + segment_length / 2] * w;
                result[position] = a + b;
                result[position + segment_length / 2] = a - b;
                w *= w_len;
            }
        }
    }
    let scale = 1.0 / n as f64;
    result.iter().map(|x| x.re * scale).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn almost_equal(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }

    const EPSILON: f64 = 1e-6;

    #[test]
    fn small_polynomial_returns_self() {
        let polynomial = vec![1.0f64, 1.0, 0.0, 2.5];
        let permutation = fast_fourier_transform_input_permutation(polynomial.len());
        let fft = fast_fourier_transform(&polynomial, &permutation);
        let ifft = inverse_fast_fourier_transform(&fft, &permutation);
        for (x, y) in ifft.iter().zip(polynomial.iter()) {
            assert!(almost_equal(*x, *y, EPSILON));
        }
    }

    #[test]
    fn square_small_polynomial() {
        let mut polynomial = vec![1.0f64, 1.0, 0.0, 2.0];
        polynomial.append(&mut vec![0.0; 4]);
        let permutation = fast_fourier_transform_input_permutation(polynomial.len());
        let mut fft = fast_fourier_transform(&polynomial, &permutation);
        fft.iter_mut().for_each(|num| *num *= *num);
        let ifft = inverse_fast_fourier_transform(&fft, &permutation);
        let expected = [1.0, 2.0, 1.0, 4.0, 4.0, 0.0, 4.0, 0.0, 0.0];
        for (x, y) in ifft.iter().zip(expected.iter()) {
            assert!(almost_equal(*x, *y, EPSILON));
        }
    }

    #[test]
    #[ignore]
    fn square_big_polynomial() {
        // This test case takes ~1050ms on my machine in unoptimized mode,
        // but it takes ~70ms in release mode.
        let n = 1 << 17; // ~100_000
        let mut polynomial = vec![1.0f64; n];
        polynomial.append(&mut vec![0.0f64; n]);
        let permutation = fast_fourier_transform_input_permutation(polynomial.len());
        let mut fft = fast_fourier_transform(&polynomial, &permutation);
        fft.iter_mut().for_each(|num| *num *= *num);
        let ifft = inverse_fast_fourier_transform(&fft, &permutation);
        let expected = (0..((n << 1) - 1)).map(|i| std::cmp::min(i + 1, (n << 1) - 1 - i) as f64);
        for (&x, y) in ifft.iter().zip(expected) {
            assert!(almost_equal(x, y, EPSILON));
        }
    }
}
