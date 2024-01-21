use std::collections::LinkedList;

pub fn strand_sort(ip: &mut LinkedList<i32>, op: &mut LinkedList<i32>) {
    if ip.is_empty() {
        return;
    }

    let mut sublist = LinkedList::new();
    sublist.push_back(ip.pop_front().unwrap());

    let mut iter = ip.split_off(0);
    while let Some(val) = iter.pop_front() {
        if val > *sublist.back().unwrap() {
            sublist.push_back(val);
        } else {
            ip.push_back(val);
        }
    }

    // Merge current sublist into output
    let mut merged = LinkedList::new();
    while !op.is_empty() || !sublist.is_empty() {
        match (op.front(), sublist.front()) {
            (Some(&op_val), Some(&sub_val)) if op_val <= sub_val => {
                merged.push_back(op.pop_front().unwrap());
            }
            (_, Some(_)) => {
                merged.push_back(sublist.pop_front().unwrap());
            }
            (Some(_), _) => {
                merged.push_back(op.pop_front().unwrap());
            }
            (None, None) => break,
        }
    }

    *op = merged;
    strand_sort(ip, op);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strand_sort() {
        let mut ip: LinkedList<i32> = LinkedList::from([10, 5, 30, 40, 2, 4, 9]);
        let mut op: LinkedList<i32> = LinkedList::new();

        strand_sort(&mut ip, &mut op);

        assert_eq!(op, LinkedList::from([2, 4, 5, 9, 10, 30, 40]));
    }

    #[test]
    fn test_strand_sort_empty() {
        let mut ip: LinkedList<i32> = LinkedList::new();
        let mut op: LinkedList<i32> = LinkedList::new();

        strand_sort(&mut ip, &mut op);

        assert_eq!(op, LinkedList::new());
    }

    #[test]
    fn test_strand_sort_single() {
        let mut ip: LinkedList<i32> = LinkedList::from([1]);
        let mut op: LinkedList<i32> = LinkedList::new();

        strand_sort(&mut ip, &mut op);

        assert_eq!(op, LinkedList::from([1]));
    }

    #[test]
    fn test_strand_sort_negative() {
        let mut ip: LinkedList<i32> = LinkedList::from([-1, -2, -3, -4, -5]);
        let mut op: LinkedList<i32> = LinkedList::new();

        strand_sort(&mut ip, &mut op);

        assert_eq!(op, LinkedList::from([-5, -4, -3, -2, -1]));
    }

    #[test]
    fn test_strand_sort_duplicates() {
        let mut ip: LinkedList<i32> = LinkedList::from([1, 1, 1, 1, 1]);
        let mut op: LinkedList<i32> = LinkedList::new();

        strand_sort(&mut ip, &mut op);

        assert_eq!(op, LinkedList::from([1, 1, 1, 1, 1]));
    }

    #[test]
    fn test_strand_sort_error() {
        let mut ip: LinkedList<i32> = LinkedList::from([1, 2, 3, 4, 5]);
        let mut op: LinkedList<i32> = LinkedList::new();

        strand_sort(&mut ip, &mut op);

        assert_ne!(op, LinkedList::from([2, 1, 3, 4, 5]));
    }

    #[test]
    fn test_strand_sort_big() {
        let mut ip: LinkedList<i32> = LinkedList::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let mut op: LinkedList<i32> = LinkedList::new();

        strand_sort(&mut ip, &mut op);

        assert_eq!(op, LinkedList::from([1, 2, 3, 4, 5, 6, 7, 8, 9]));
    }

    #[test]
    fn test_strand_sort_big_reverse() {
        let mut ip: LinkedList<i32> = LinkedList::from([9, 8, 7, 6, 5, 4, 3, 2, 1]);
        let mut op: LinkedList<i32> = LinkedList::new();

        strand_sort(&mut ip, &mut op);

        assert_eq!(op, LinkedList::from([1, 2, 3, 4, 5, 6, 7, 8, 9]));
    }
}
