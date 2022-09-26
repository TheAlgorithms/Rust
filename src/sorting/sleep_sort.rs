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
        let mut arr = vec![3, 4, 2, 1, 7];
        let res = sleep_sort(&mut arr);
        assert_eq!(res, &[1, 2, 3, 4, 7]);
    }

    #[test]
    fn repeated_elements() {
        let mut arr = vec![542, 542, 542, 542];
        let res = sleep_sort(&mut arr);
        assert_eq!(res, &[542, 542, 542, 542]);
    }

    #[test]
    fn random_elements() {
        let mut arr = vec![
            52, 958, 385, 130, 687, 86, 480, 329, 269, 648, 112, 286, 222, 844, 463, 982, 571, 104,
            491, 223, 791, 90, 43, 884, 518, 680, 347, 822, 505, 778, 62, 743, 775, 8, 357, 532,
            53, 680, 32, 271, 267, 306, 20, 915, 374, 477, 272, 638, 18, 299,
        ];
        let res = sleep_sort(&mut arr);
        assert_eq!(
            res,
            &[
                8, 18, 20, 32, 43, 52, 53, 62, 86, 90, 104, 112, 130, 222, 223, 267, 269, 271, 272,
                286, 299, 306, 329, 347, 357, 374, 385, 463, 477, 480, 491, 505, 518, 532, 571,
                638, 648, 680, 680, 687, 743, 775, 778, 791, 822, 844, 884, 915, 958, 982
            ]
        );
    }
}
