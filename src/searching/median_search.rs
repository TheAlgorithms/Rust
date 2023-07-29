use std::cmp::min;

pub fn median_of_medians(array: &[i32], idx: i32) -> i32 {
    let mut a: Vec<i32> = array.to_owned();
    let mut m: Vec<i32> = Vec::new();
    let r: i32 = a.len() as i32;

    let mut i: i32 = 0;
    loop {
        if i >= r {
            break;
        }
        a[i as usize..min(r, i + 5) as usize].sort();
        let mid = (i + min(r, i + 5)) / 2;
        m.push(a[mid as usize]);
        i += 5;
    }

    let sz: i32 = m.len() as i32;
    let pivot: i32 = if sz <= 5 {
        m.sort();
        m[((sz - 1) / 2) as usize]
    } else {
        median_of_medians(&m, idx)
    };

    let mut low: Vec<i32> = Vec::new();
    let mut high: Vec<i32> = Vec::new();

    for i in 0..r {
        match a[i as usize] {
            x if x < pivot => low.push(x),
            x if x > pivot => high.push(x),
            _ => {}
        }
    }

    let k = low.len() as i32;
    match k {
        x if x > idx => median_of_medians(&low, idx),
        x if x < idx => median_of_medians(&high, idx - k - 1),
        _ => pivot,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut a: Vec<i32> = vec![25, 21, 98, 100, 76, 22, 43, 60, 89, 87];
        let i = 3;
        assert_eq!(median_of_medians(&mut a, i), a[6]);
    }

    #[test]
    fn test2() {
        let mut a: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        let i = 4;
        assert_eq!(median_of_medians(&mut a, i), a[4]);
    }

    #[test]
    fn test3() {
        let mut a: Vec<i32> = vec![1, 2, 3, 4, 5, 1000, 8, 9, 99];
        let i = 3;
        assert_eq!(median_of_medians(&mut a, i), a[3]);
    }
}
