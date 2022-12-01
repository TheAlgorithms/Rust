use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn sleep_sort(vec: &[usize]) -> Vec<usize> {
    let (tx, rx) = mpsc::channel();

    for &x in vec.iter() {
        let tx = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis((20 * x) as u64));
            tx.send(x).expect("panic");
        });
    }
    drop(tx);

    let mut sorted_list: Vec<usize> = Vec::new();
    while let Ok(x) = rx.recv() {
        sorted_list.push(x)
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
        let mut arr = vec![1, 2, 3, 4];
        let res = sleep_sort(&mut arr);
        assert_eq!(res, &[1, 2, 3, 4]);
    }

    #[test]
    fn unsorted_array() {
        let mut arr = vec![3, 4, 2, 1];
        let res = sleep_sort(&mut arr);
        assert_eq!(res, &[1, 2, 3, 4]);
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
        let mut arr = vec![5, 3, 7, 10, 1, 0, 8];
        let res = sleep_sort(&mut arr);
        assert_eq!(res, &[0, 1, 3, 5, 7, 8, 10]);
    }
}
