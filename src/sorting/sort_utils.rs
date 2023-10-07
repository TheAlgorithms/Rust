use rand::Rng;
use std::time::Instant;

#[cfg(test)]
pub fn generate_random_vec(n: u32, range_l: i32, range_r: i32) -> Vec<i32> {
    let mut arr = Vec::<i32>::with_capacity(n as usize);
    let mut rng = rand::thread_rng();
    let mut count = n;

    while count > 0 {
        arr.push(rng.gen_range(range_l..range_r + 1));
        count -= 1;
    }

    arr
}

#[cfg(test)]
pub fn generate_nearly_ordered_vec(n: u32, swap_times: u32) -> Vec<i32> {
    let mut arr: Vec<i32> = (0..n as i32).collect();
    let mut rng = rand::thread_rng();

    let mut count = swap_times;

    while count > 0 {
        arr.swap(rng.gen_range(0..n as usize), rng.gen_range(0..n as usize));
        count -= 1;
    }

    arr
}

#[cfg(test)]
pub fn generate_ordered_vec(n: u32) -> Vec<i32> {
    generate_nearly_ordered_vec(n, 0)
}

#[cfg(test)]
pub fn generate_reverse_ordered_vec(n: u32) -> Vec<i32> {
    let mut arr = generate_ordered_vec(n);
    arr.reverse();
    arr
}

#[cfg(test)]
pub fn generate_repeated_elements_vec(n: u32, unique_elements: u8) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let v = rng.gen_range(0..n as i32);
    generate_random_vec(n, v, v + unique_elements as i32)
}

#[cfg(test)]
pub fn log_timed<F>(test_name: &str, f: F)
where
    F: FnOnce(),
{
    let before = Instant::now();
    f();
    println!("Elapsed time of {:?} is {:?}", test_name, before.elapsed());
}
