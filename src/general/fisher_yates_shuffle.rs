pub fn fisher_yates_shuffle(array: &mut [i32]) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let len = array.len();

    for i in 0..(len - 2) {
        let j = rng.gen_range(i..len);
        array.swap(i, j);
    }
}
