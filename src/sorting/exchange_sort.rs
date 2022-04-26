// sorts through swapping the first value until it is at the right position, and repeating for all the following.

pub fn exchange_sort(arr: &mut [i32]) {
    let length = arr.len();
    for number1 in 0..length {
        for number2 in (number1 + 1)..length {
            if arr[number2] < arr[number1] {
                arr.swap(number1, number2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;
    #[test]
    fn it_works() {
        let mut arr1 = [6, 5, 4, 3, 2, 1];
        exchange_sort(&mut arr1);
        assert!(is_sorted(&arr1));
        arr1 = [12, 343, 21, 90, 3, 21];
        exchange_sort(&mut arr1);
        assert!(is_sorted(&arr1));
        let mut arr2 = [1];
        exchange_sort(&mut arr2);
        assert!(is_sorted(&arr2));
        let mut arr3 = [213, 542, 90, -23412, -32, 324, -34, 3324, 54];
        exchange_sort(&mut arr3);
        assert!(is_sorted(&arr3));
    }
}
