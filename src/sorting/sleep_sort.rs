use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn sleep_sort(vec: &[usize]) -> Vec<usize> {
    let len = vec.len();
    let (tx, rx) = mpsc::channel();

    for &x in vec.iter() {
        let tx: mpsc::Sender<usize> = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis((20 * x) as u64));
            tx.send(x).expect("panic");
        });
    }
    let mut sorted_list: Vec<usize> = Vec::new();

    for _ in 0..len {
        sorted_list.push(rx.recv().unwrap())
    }

    sorted_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let res = sleep_sort(&[]);
        assert_eq!(res, &[]);
    }

    #[test]
    fn single_element() {
        let res = sleep_sort(&[1]);
        assert_eq!(res, &[1]);
    }

    #[test]
    fn sorted_array() {
        let res = sleep_sort(&[1, 2, 3, 4]);
        assert_eq!(res, &[1, 2, 3, 4]);
    }

    #[test]
    fn unsorted_array() {
        let res = sleep_sort(&[3, 4, 2, 1]);
        assert_eq!(res, &[1, 2, 3, 4]);
    }

    #[test]
    fn odd_number_of_elements() {
        let res = sleep_sort(&[3, 1, 7]);
        assert_eq!(res, &[1, 3, 7]);
    }

    #[test]
    fn repeated_elements() {
        let res = sleep_sort(&[1, 1, 1, 1]);
        assert_eq!(res, &[1, 1, 1, 1]);
    }

    #[test]
    fn random_elements() {
        let res = sleep_sort(&[5, 3, 7, 10, 1, 0, 8]);
        assert_eq!(res, &[0, 1, 3, 5, 7, 8, 10]);
    }
}
