use crate::data_structures::Heap;
use std::cmp::{Ord, Ordering};

/// Returns k-th smallest element of an array.
/// Time complexity is stably O(nlog(k)) in all cases
/// Extra space is required to maintain the heap, and it doesn't
/// mutate the input list.
///
/// It is preferrable to the partition-based algorithm in cases when
/// we want to maintain the kth smallest element dynamically against
/// a stream of elements. In that case, once the heap is built, further
/// operation's complexity is O(log(k)).
pub fn kth_smallest_heap<T>(input: &[T], k: usize) -> Option<T>
where
    T: Ord + Copy,
{
    if input.len() < k {
        return None;
    }

    // heap will maintain the kth smallest elements
    // seen so far, when new elements, E_new arrives,
    // it is compared with the largest element of the
    // current Heap E_large, which is the current kth
    // smallest elements.
    // if E_new > E_large, then E_new cannot be the kth
    // smallest because there are already k elements smaller
    // than it
    // otherwise, E_large cannot be the kth smallest, and should
    // be removed from the heap and E_new should be added
    let mut heap = Heap::new_max();

    // first k elements goes to the heap as the baseline
    for &val in input.iter().take(k) {
        heap.add(val);
    }

    for &val in input.iter().skip(k) {
        // compare new value to the current kth smallest value
        let cur_big = heap.pop().unwrap(); // heap.pop() can't be None
        match val.cmp(&cur_big) {
            Ordering::Greater => {
                heap.add(cur_big);
            }
            _ => {
                heap.add(val);
            }
        }
    }

    heap.pop()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let zero: [u8; 0] = [];
        let first = kth_smallest_heap(&zero, 1);

        assert_eq!(None, first);
    }

    #[test]
    fn one_element() {
        let one = [1];
        let first = kth_smallest_heap(&one, 1);

        assert_eq!(1, first.unwrap());
    }

    #[test]
    fn many_elements() {
        // 0 1 3 4 5 7 8 9 9 10 12 13 16 17
        let many = [9, 17, 3, 16, 13, 10, 1, 5, 7, 12, 4, 8, 9, 0];

        let first = kth_smallest_heap(&many, 1);
        let third = kth_smallest_heap(&many, 3);
        let sixth = kth_smallest_heap(&many, 6);
        let fourteenth = kth_smallest_heap(&many, 14);

        assert_eq!(0, first.unwrap());
        assert_eq!(3, third.unwrap());
        assert_eq!(7, sixth.unwrap());
        assert_eq!(17, fourteenth.unwrap());
    }
}
