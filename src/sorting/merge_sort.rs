pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let mut mem = arr.to_vec();
    _merge_sort(arr, &mut mem);
}

fn _merge_sort<T: Ord + Clone>(arr: &mut [T], mem: &mut [T]) {
    assert_eq!(arr.len(), mem.len());
    match arr.len() {
        0...1 => (),
        len => {
            let (left_mem, right_mem) = mem.split_at_mut(len / 2);
            {
                let (left, right) = arr.split_at_mut(len / 2);
                _merge_sort(left_mem, left);
                _merge_sort(right_mem, right);
            }
            merge(arr, left_mem, right_mem);
        }
    }
}

fn merge<T: Ord + Clone>(dest: &mut [T], left: &[T], right: &[T]) {
    let mut lit = left.iter().peekable();
    let mut rit = right.iter().peekable();

    for elem in dest.iter_mut() {
        *elem = match (lit.peek(), rit.peek()) {
            (Some(&l), Some(&r)) => {
                if l < r {
                    lit.next().unwrap();
                    l.clone()
                } else {
                    rit.next().unwrap();
                    r.clone()
                }
            }
            (Some(&item), None) => {
                lit.next().unwrap();
                item.clone()
            }
            (None, Some(&item)) => {
                rit.next().unwrap();
                item.clone()
            }
            _ => return (),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn merge() {
        use sorting::merge_sort;

        let mut arr: [u8; 0] = [];
        merge_sort(&mut arr);
        assert_eq!([] as [u8; 0], arr);

        let mut arr = [9, 2, 4, 8, 2, 6];
        merge_sort(&mut arr);
        assert_eq!([2, 2, 4, 6, 8, 9], arr);
    }
}
