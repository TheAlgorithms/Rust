/// Sort a mutable slice using merge sort.
///
/// Merge sort is an in-place O(n log n) sorting algorithm on a every case. \
/// It is based on a max subsequence of slices based on a central pivot, full sequence is splitten within n subsequence
/// and every sequence is being ordered alone, then merged all together.
///
/// # Merge Sort

fn merge_sort(nums: &mut std::vec::Vec<usize>) {
    if nums.len() > 1 {
        let mid: usize = nums.len() / 2;
        
        let mut left: Vec<usize> = nums[0..mid].to_vec();
        let mut right: Vec<usize> = nums[mid..nums.len()].to_vec();

        merge_sort(&mut left);
        merge_sort(&mut right);

        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut k: usize = 0;

        while i < left.len() && j < right.len() {
            if left[i] < right[j] {
                nums[k] = left[i];
                i += 1;
            } else {
                nums[k] = right[j];
                j += 1;
            }
            k += 1;
        }

        while i < left.len() {
            nums[k] = left[i];
            i += 1;
            k += 1;
        }

        while j < right.len() {
            nums[k] = right[j];
            j += 1;
            k += 1;
        }
    }
}
