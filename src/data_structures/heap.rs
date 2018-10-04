use std::marker::Sized;

pub struct MinHeap<T> {
    arr: Vec<T>,
}

impl<T: Sized + Copy + Ord> MinHeap<T> {
    pub fn new() -> MinHeap<T> {
        MinHeap { arr: vec![] }
    }

    pub fn size(&self) -> usize {
        self.arr.len()
    }

    pub fn insert(&mut self, item: T) -> () {
        self.arr.push(item);

        let mut index = self.arr.len() - 1;
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.arr[parent] > self.arr[index] {
                self.arr.swap(parent, index);
            }
            index = parent;
        }
    }

    // Moves the element at `index` to it's correct position
    // Assumes the left and right subtrees as min-heaps
    fn heapify(&mut self, index: usize) -> () {
        let len = self.arr.len();

        if index > len - 1 {
            return ();
        }

        let left_child = index * 2 + 1;
        let right_child = left_child + 1;

        if left_child < len && self.arr[index] > self.arr[left_child] {
            self.arr.swap(left_child, index);
            self.heapify(left_child);
        }
        if right_child < len && self.arr[index] > self.arr[right_child] {
            self.arr.swap(right_child, index);
            self.heapify(right_child);
        }
    }

    pub fn extract_min(&mut self) -> Option<T> {
        if self.arr.len() < 1 {
            return None;
        }

        let item = self.arr.swap_remove(0);
        self.heapify(0);

        Some(item)
    }
}

#[cfg(test)]
mod test {
    use super::MinHeap;
    #[test]
    fn heap_with_integers() {
        let mut heap = MinHeap::new();
        let items = vec![10, 50, -10, 100, 1, 40, 3];
        for item in items.iter().cloned() {
            heap.insert(item);
        }
        assert_eq!(heap.extract_min().unwrap(), -10);
        assert_eq!(heap.size(), items.len() - 1);
        assert_eq!(heap.extract_min().unwrap(), 1);
        assert_eq!(heap.size(), items.len() - 2);
        assert_eq!(heap.extract_min().unwrap(), 3);
    }

    fn heap_with_chars() {
        let mut heap = MinHeap::new();
        let items = vec!["z", "f", "h", "y", "a", "l", "e"];
        for item in items.iter().cloned() {
            heap.insert(item);
        }
        assert_eq!(heap.extract_min().unwrap(), "a");
        assert_eq!(heap.size(), items.len() - 1);
        assert_eq!(heap.extract_min().unwrap(), "e");
        assert_eq!(heap.size(), items.len() - 2);
        assert_eq!(heap.extract_min().unwrap(), "f");
    }
}
