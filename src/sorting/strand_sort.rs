use std::collections::LinkedList;

pub fn strand_sort(list: &LinkedList<i32>) -> LinkedList<i32> {
    if list.is_empty() {
        return LinkedList::new();
    }

    let mut ip = list.iter().cloned().collect::<LinkedList<i32>>();
    let mut op = LinkedList::new();

    while !ip.is_empty() {
        let mut sublist = LinkedList::new();
        sublist.push_back(ip.pop_front().unwrap());

        let mut temp_list = LinkedList::new();
        while let Some(val) = ip.pop_front() {
            if val >= *sublist.back().unwrap() {
                sublist.push_back(val);
            } else {
                temp_list.push_back(val);
            }
        }

        let mut merged = LinkedList::new();
        while let (Some(&op_val), Some(&sub_val)) = (op.front(), sublist.front()) {
            if op_val <= sub_val {
                merged.push_back(op.pop_front().unwrap());
            } else {
                merged.push_back(sublist.pop_front().unwrap());
            }
        }
        merged.append(&mut op);
        merged.append(&mut sublist);

        op = merged;
        ip = temp_list;
    }

    op
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::LinkedList;

    #[test]
    fn test_strand_sort_basic() {
        let ip = LinkedList::from([10, 5, 30, 40, 2, 4, 9]);
        let result = strand_sort(&ip);
        assert_eq!(result, LinkedList::from([2, 4, 5, 9, 10, 30, 40]));
    }

    #[test]
    fn test_strand_sort_empty() {
        let ip = LinkedList::new();
        let result = strand_sort(&ip);
        assert_eq!(result, LinkedList::new());
    }

    #[test]
    fn test_strand_sort_single_element() {
        let ip = LinkedList::from([1]);
        let result = strand_sort(&ip);
        assert_eq!(result, LinkedList::from([1]));
    }

    #[test]
    fn test_strand_sort_negative_numbers() {
        let ip = LinkedList::from([-1, -2, -3, -4, -5]);
        let result = strand_sort(&ip);
        assert_eq!(result, LinkedList::from([-5, -4, -3, -2, -1]));
    }

    #[test]
    fn test_strand_sort_with_duplicates() {
        let ip = LinkedList::from([1, 1, 1, 1, 1]);
        let result = strand_sort(&ip);
        assert_eq!(result, LinkedList::from([1, 1, 1, 1, 1]));
    }

    #[test]
    fn test_strand_sort_sorted_input() {
        let ip = LinkedList::from([1, 2, 3, 4, 5]);
        let result = strand_sort(&ip);
        assert_eq!(result, LinkedList::from([1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_strand_sort_reverse_sorted_input() {
        let ip = LinkedList::from([5, 4, 3, 2, 1]);
        let result = strand_sort(&ip);
        assert_eq!(result, LinkedList::from([1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_strand_sort_interleaved_merge() {
        let ip = LinkedList::from([4, 1, 5, 2, 6, 3]);
        let result = strand_sort(&ip);
        assert_eq!(result, LinkedList::from([1, 2, 3, 4, 5, 6]));
    }
}
