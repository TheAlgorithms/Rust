// https://en.wikipedia.org/wiki/Quickselect

fn partition(list: &mut [i32], left: usize, right: usize, pivot_index: usize) -> usize {
    let pivot_value = list[pivot_index];
    list.swap(pivot_index, right);  // Move pivot to end
    let mut store_index = left;
    for i in left..(right + 1) { 
        if list[i] < pivot_value {
            list.swap(store_index, i);
            store_index += 1;
        }
        list.swap(right, store_index);  // Move pivot to its final place
    }
    return store_index
}

pub fn quick_select(list: &mut [i32], left: usize, right: usize, index: usize) -> i32 {
    if left == right {   // If the list contains only one element,
        return list[left];
    }// return that element
    let mut pivot_index = (left + right) / 2;    // select a pivotIndex between left and right
    pivot_index = partition(list, left, right, pivot_index);
    // The pivot is in its final sorted position
    if index == pivot_index {
        return list[index];
    }
    else if index < pivot_index {
        return quick_select(list, left, right - 1, index);
    }
    else {
        return quick_select(list, left, right + 1, index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut arr1 = [2, 3, 4, 5];
        assert_eq!(quick_select(&mut arr1, 0, 3, 2), 3);
    }    
}
