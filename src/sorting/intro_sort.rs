// Intro Sort (Also known as Introspective Sort)
// Introspective Sort is hybrid sort (Quick Sort + Heap Sort + Insertion Sort)
// https://en.wikipedia.org/wiki/Introsort
fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

fn heap_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();

    // Build a max-heap
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    // Extract elements from the heap one by one
    for i in (0..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

pub fn intro_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    let max_depth = (2.0 * len as f64).log2() as usize + 1;

    fn intro_sort_recursive<T: Ord>(arr: &mut [T], max_depth: usize) {
        let len = arr.len();

        if len <= 16 {
            insertion_sort(arr);
        } else if max_depth == 0 {
            heap_sort(arr);
        } else {
            let pivot = partition(arr);
            intro_sort_recursive(&mut arr[..pivot], max_depth - 1);
            intro_sort_recursive(&mut arr[pivot + 1..], max_depth - 1);
        }
    }

    fn partition<T: Ord>(arr: &mut [T]) -> usize {
        let len = arr.len();
        let pivot_index = len / 2;
        arr.swap(pivot_index, len - 1);

        let mut i = 0;
        for j in 0..len - 1 {
            if arr[j] <= arr[len - 1] {
                arr.swap(i, j);
                i += 1;
            }
        }

        arr.swap(i, len - 1);
        i
    }

    intro_sort_recursive(arr, max_depth);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intro_sort() {
        // Test with integers
        let mut arr1 = vec![67, 34, 29, 15, 21, 9, 99];
        intro_sort(&mut arr1);
        assert_eq!(arr1, vec![9, 15, 21, 29, 34, 67, 99]);

        // Test with strings
        let mut arr2 = vec!["sydney", "london", "tokyo", "beijing", "mumbai"];
        intro_sort(&mut arr2);
        assert_eq!(arr2, vec!["beijing", "london", "mumbai", "sydney", "tokyo"]);

        // Test with an empty array
        let mut arr3: Vec<i32> = vec![];
        intro_sort(&mut arr3);
        assert_eq!(arr3, vec![]);
    }
}
