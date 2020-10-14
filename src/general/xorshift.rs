/*
Simple pseudo-random number generator usefull to avoid import of the 'rand' crate
https://en.wikipedia.org/wiki/Xorshift

Starting point: https://users.rust-lang.org/t/random-number-without-using-the-external-crate/17260/10
*/

pub struct Rand {
    x: usize,
    y: usize,
    z: usize,
    w: usize,
}

impl Rand {
    pub fn new(seed: usize) -> Self {
        Rand {
            x: 123_456_789 ^ seed,
            y: 362_436_069 ^ seed,
            z: 521_288_629,
            w: 88_675_123,
        }
    }

    pub fn randint(&mut self) -> usize {
        let t = self.x ^ self.x.wrapping_shl(11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w ^= self.w.wrapping_shr(19) ^ t ^ t.wrapping_shr(8);
        self.w
    }

    pub fn shuffle<T>(&mut self, a: &mut [T]) {
        if a.is_empty() {
            return;
        }
        let mut i = a.len() - 1;
        while i > 0 {
            let j = (self.randint() as usize) % (i + 1);
            a.swap(i, j);
            i -= 1;
        }
    }

    pub fn rand_range(&mut self, a: isize, b: isize) -> isize {
        a + (self.randint() % (b - a) as usize) as isize
    }

    pub fn rand_range_usize(&mut self, a: usize, b: usize) -> usize {
        a + (self.randint() % (b - a))
    }

    pub fn rand_float(&mut self) -> f64 {
        (self.randint() as f64) / (<u32>::max_value() as f64)
    }

    pub fn rand_float_normal(&mut self) -> f32 {
        (self.randint() as f32 % 100.0) / 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rand_output_distribution() {
        // Simple number distribution calculation using first 10 digits (0 to 9)
        // to verify random distribution of the output
        let mut rng = Rand::new(0);
        let container: Vec<usize> = (0..500_000).map(|_| rng.rand_range_usize(0, 10)).collect();
        for i in 0..9 {
            assert_eq!(
                (container.iter().filter(|&n| *n == i).count() as f32 / 500_000.0) < 0.15,
                true
            );
        }
    }

    #[test]
    fn rand_float_distribution() {
        // Simple number distribution calculation using 0.0 to 1.0 values
        // to verify random distribution of the output
        let mut rng = Rand::new(0);
        let container: Vec<f32> = (0..500_000).map(|_| rng.rand_float_normal()).collect();
        assert_eq!(
            (container.iter().filter(|&n| *n > 0.5).count() as f32 / 500_000.0) < 0.5,
            true
        );
        assert_eq!(
            (container.iter().filter(|&n| *n < 0.5).count() as f32 / 500_000.0) > 0.5,
            true
        );
    }
}
