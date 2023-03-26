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
        assert!(crate::sorting::is_sorted(&res));
    }

    #[test]
    fn single_element() {
        let res = sleep_sort(&[1]);
        assert!(crate::sorting::is_sorted(&res));
    }

    #[test]
    fn sorted_array() {
        let res = sleep_sort(&[1, 2, 3, 4]);
        assert!(crate::sorting::is_sorted(&res));
    }

    #[test]
    fn unsorted_array() {
        let res = sleep_sort(&[3, 4, 2, 1]);
        assert!(crate::sorting::is_sorted(&res));
    }

    #[test]
    fn odd_number_of_elements() {
        let res = sleep_sort(&[3, 1, 7]);
        assert!(crate::sorting::is_sorted(&res));
    }

    #[test]
    fn repeated_elements() {
        let res = sleep_sort(&[1, 1, 1, 1]);
        assert!(crate::sorting::is_sorted(&res));
    }

    #[test]
    fn random_elements() {
        let res = sleep_sort(&[5, 3, 7, 10, 1, 0, 8]);
        assert!(crate::sorting::is_sorted(&res));
    }
}
