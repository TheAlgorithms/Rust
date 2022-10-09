use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn sleep_sort(vec: &[usize]) -> Vec<usize> {
    let len = vec.len();
    let (tx, rx) = mpsc::channel();

    for &x in vec.iter() {
        let tx: mpsc::Sender<usize> = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis((10 * x) as u64));
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
        let mut arr: Vec<usize> = Vec::new();
        let res = sleep_sort(&mut arr);
        assert_eq!(res, &[]);
    }

    #[test]
    fn single_element() {
        let mut arr = vec![1];
        let res = sleep_sort(&mut arr);
        assert_eq!(res, &[1]);
    }

    #[test]
    fn sorted_array() {
        let mut arr = vec![0, 2, 4, 6];
        let res = sleep_sort(&mut arr);
        assert_eq!(res, &[0, 2, 4, 6]);
    }

    #[test]
    fn unsorted_array() {
        let mut arr = vec![4, 6, 2, 0];
        let res = sleep_sort(&mut arr);
        assert_eq!(res, &[0, 2, 4, 6]);
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr = vec![3, 1, 7];
        let res = sleep_sort(&mut arr);
        assert_eq!(res, &[1, 3, 7]);
    }

    #[test]
    fn repeated_elements() {
        let mut arr = vec![1, 1, 1, 1];
        let res = sleep_sort(&mut arr);
        assert_eq!(res, &[1, 1, 1, 1]);
    }

    #[test]
    fn random_elements() {
        let mut arr = vec![11, 7, 14, 20, 2, 0, 17];
        let res = sleep_sort(&mut arr);
        assert_eq!(res, &[0, 2, 7, 11, 14, 17, 20]);
    }
}
