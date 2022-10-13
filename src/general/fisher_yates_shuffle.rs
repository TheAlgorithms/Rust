use std::time::{SystemTime, UNIX_EPOCH};

use crate::math::PCG32;

const DEFAULT: u64 = 4294967296;

fn gen_range(range: usize, generator: &mut PCG32) -> usize {
    generator.get_u64() as usize % range
}

pub fn fisher_yates_shuffle(array: &mut [i32]) {
    let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_millis() as u64,
        Err(_) => DEFAULT,
    };

    let mut random_generator = PCG32::new_default(seed);

    let len = array.len();

    for i in 0..(len - 2) {
        let r = gen_range(len - i, &mut random_generator);
        array.swap(i, i + r);
    }
}
