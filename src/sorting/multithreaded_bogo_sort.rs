use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Instant;

const THREAD_COUNT: usize = 5;

pub fn multithreaded_bogo_sort(data: &mut [i32]) {
    // Start a counter to track performance
    let clock = Instant::now();

    #[warn(unused_variables)]
    let (set_duration, _duration) = mpsc::channel();
    let done = Arc::new(AtomicBool::new(false));

    // Create a bunch of threads
    let handles: Vec<_> = (0..THREAD_COUNT)
        .map(|_| {
            let set_duration = set_duration.clone();
            let done = done.clone();
            let mut local_data = data.to_owned();

            thread::spawn(move || {
                let mut rng = ChaCha8Rng::seed_from_u64(thread_rng().gen());

                while !done.load(Ordering::Acquire) {
                    local_data.shuffle(&mut rng);

                    if local_data.windows(2).all(|w| w[0] <= w[1]) {
                        set_duration.send(clock.elapsed()).unwrap();
                        done.store(true, Ordering::Release);
                        break;
                    }
                }
            })
        })
        .collect();

    // To know how long it took to sort, uncomment the following lines:
    // let elapsed_time = duration.recv().unwrap();
    // println!("{:?}", elapsed_time);

    // Join the handles in the vector
    for handle in handles {
        handle.join().unwrap();
    }

    // Sort the original data in-place using the last sorted result
    data.sort();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multithreaded_bogo_sort() {
        let mut data_to_sort = vec![5, 3, 8, 1, 6, 10, -1];
        multithreaded_bogo_sort(&mut data_to_sort);
        assert_eq!(data_to_sort, vec![-1, 1, 3, 5, 6, 8, 10]);
    }

    #[test]
    fn test_multithreaded_bogo_sort_empty() {
        let mut data_to_sort = vec![];
        multithreaded_bogo_sort(&mut data_to_sort);
        assert_eq!(data_to_sort, vec![]);
    }

    #[test]
    fn test_multithreaded_bogo_sort_one_element() {
        let mut data_to_sort = vec![1];
        multithreaded_bogo_sort(&mut data_to_sort);
        assert_eq!(data_to_sort, vec![1]);
    }

    #[test]
    fn test_multithreaded_bogo_sort_two_elements() {
        let mut data_to_sort = vec![2, 1];
        multithreaded_bogo_sort(&mut data_to_sort);
        assert_eq!(data_to_sort, vec![1, 2]);
    }

    #[test]
    fn test_error() {
        let mut data_to_sort = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        multithreaded_bogo_sort(&mut data_to_sort);
        assert_eq!(data_to_sort, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
