pub fn smooth_sort(nums: &mut [i32]) {
    let n = nums.len();
    if n <= 1 {
        return;
    }
    
    let mut heap_sizes = Vec::new();
    let mut heap_count = 0;

    for i in 0..n {
        add_to_heap(nums, i, &mut heap_sizes, &mut heap_count);
    }
    for i in (0..n).rev() {
        remove_from_heap(nums, i, &mut heap_sizes, &mut heap_count);
    }
}

fn add_to_heap(nums: &mut [i32], index: usize, sizes: &mut Vec<usize>, heaps: &mut usize) {
    if *heaps >= 2 && sizes[*heaps - 2] == sizes[*heaps - 1] + 1 {
        sizes[*heaps - 2] += 1;
        sizes.pop();
        *heaps -= 1;
    } else if *heaps >= 2 && sizes[*heaps - 1] == sizes[*heaps - 2] + 1 {
        sizes[*heaps - 1] += 1;
    } else {
        sizes.push(1);
    }
    heapify(nums, index, sizes, *heaps);
}

fn remove_from_heap(nums: &mut [i32], index: usize, sizes: &mut Vec<usize>, heaps: &mut usize) {
    let size = sizes.pop().unwrap();
    *heaps -= 1;
    if size >= 2 {
        sizes.push(size - 1);
        sizes.push(size - 2);
        *heaps += 2;
        heapify(nums, index - size + 1, sizes, *heaps - 2);
        heapify(nums, index - 1, sizes, *heaps - 1);
    }
}

fn heapify(nums: &mut [i32], index: usize, sizes: &[usize], mut heaps: usize) {
    let mut current = index;
    let mut heap_size = sizes[heaps];

    while heaps > 1 {
        if heap_size > current {
            break;
        }

        let left_child = current.saturating_sub(heap_size);
        let right_child = current.saturating_sub(1);

        if left_child < nums.len() && nums[current] < nums[left_child] {
            nums.swap(current, left_child);
            current = left_child;
        } else if right_child < nums.len() && nums[current] < nums[right_child] {
            nums.swap(current, right_child);
            current = right_child;
        } else {
            break;
        }

        heaps -= 1;
        heap_size -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smooth_sort_example_1() {
        let mut arr = vec![3, 5, 2, 1, 6, 4];
        smooth_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn smooth_sort_example_2() {
        let mut arr = vec![4, 1, 3, 5, 2];
        smooth_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn smooth_sort_repeated_elements() {
        let mut arr = vec![5, 5, 5, 5];
        smooth_sort(&mut arr);
        assert_eq!(arr, vec![5, 5, 5, 5]);
    }

    #[test]
    fn smooth_sort_large_elements() {
        let mut arr = vec![100, 200, 5, 10, 15];
        smooth_sort(&mut arr);
        assert_eq!(arr, vec![5, 10, 15, 100, 200]);
    }
}
