// From Wikipedia: Pigeonhole sorting is a sorting algorithm that is suitable for sorting lists of elements where the number of elements (n) and the length of the range of possible key values (N) are approximately the same. It requires O(n + N) time.

pub fn pigeonhole_sort(array: &mut [i32]) {
    if let (Some(min), Some(max)) = (array.iter().min(), array.iter().max()) {
        let holes_range: usize = (max - min + 1) as usize;
        let mut holes = vec![0; holes_range];
        let mut holes_repeat = vec![0; holes_range];
        for i in array.iter() {
            let index = *i - min;
            holes[index as usize] = *i;
            holes_repeat[index as usize] += 1;
        }
        let mut index = 0;
        for i in 0..holes_range {
            while holes_repeat[i] > 0 {
                array[index] = holes[i];
                index += 1;
                holes_repeat[i] -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn test1() {
        let mut arr1 = [3, 3, 3, 1, 2, 6, 5, 5, 5, 4, 1, 6, 3];
        pigeonhole_sort(&mut arr1);
        assert!(is_sorted(&arr1));
        let mut arr2 = [6, 5, 4, 3, 2, 1];
        pigeonhole_sort(&mut arr2);
        assert!(is_sorted(&arr2));
    }
}
