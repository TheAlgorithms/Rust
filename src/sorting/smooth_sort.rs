pub fn smooth_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }

    let mut leonardo_heap_sizes = Vec::new();
    let mut num_of_heaps = 0;

    for i in 0..nums.len() {
        add_to_leonardo_heap(nums, i, &mut leonardo_heap_sizes, &mut num_of_heaps);
    }

    for i in (0..nums.len()).rev() {
        remove_from_leonardo_heap(nums, i, &mut leonardo_heap_sizes, &mut num_of_heaps);
    }
}

fn add_to_leonardo_heap(nums: &mut [i32], index: usize, sizes: &mut Vec<usize>, heaps: &mut usize) {
    if *heaps >= 2 && sizes[*heaps - 2] == sizes[*heaps - 1] + 1 {
        sizes[*heaps - 2] += 1;
        sizes.pop();
        *heaps -= 1;
    } else if *heaps >= 2 && sizes[*heaps - 1] == sizes[*heaps - 2] + 1 {
        sizes[*heaps - 1] += 1;
    } else {
        sizes.push(1);
    }
    heapify_leonardo(nums, index, sizes, *heaps);
}

fn remove_from_leonardo_heap(
    nums: &mut [i32],
    index: usize,
    sizes: &mut Vec<usize>,
    heaps: &mut usize,
) {
    let size = sizes.pop().unwrap();
    *heaps -= 1;
    if size >= 2 {
        sizes.push(size - 1);
        sizes.push(size - 2);
        *heaps += 2;

        if index + 1 >= size {
            heapify_leonardo(nums, index.saturating_sub(size - 1), sizes, *heaps - 2);
            heapify_leonardo(nums, index.saturating_sub(1), sizes, *heaps - 1);
        }
    }
}

fn heapify_leonardo(nums: &mut [i32], index: usize, sizes: &[usize], mut heaps: usize) {
    let mut current = index;
    while heaps > 1 && heaps <= sizes.len() {
        let heap_size = sizes[heaps - 1];

        if current < heap_size {
            break;
        }

        let left_child = current.saturating_sub(heap_size);
        let right_child = current.saturating_sub(1);

        if nums[current] < nums[left_child] {
            nums.swap(current, left_child);
            current = left_child;
        } else if nums[current] < nums[right_child] {
            nums.swap(current, right_child);
            current = right_child;
        } else {
            break;
        }

        heaps -= 1;
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