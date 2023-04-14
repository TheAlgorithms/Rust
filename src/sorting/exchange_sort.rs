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
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;
    #[test]
    fn it_works() {
        let mut arr1 = [6, 5, 4, 3, 2, 1];
        let cloned = arr1;
        exchange_sort(&mut arr1);
        assert!(is_sorted(&arr1) && have_same_elements(&arr1, &cloned));
        arr1 = [12, 343, 21, 90, 3, 21];
        let cloned = arr1;
        exchange_sort(&mut arr1);
        assert!(is_sorted(&arr1) && have_same_elements(&arr1, &cloned));
        let mut arr2 = [1];
        let cloned = arr2;
        exchange_sort(&mut arr2);
        assert!(is_sorted(&arr2) && have_same_elements(&arr2, &cloned));
        let mut arr3 = [213, 542, 90, -23412, -32, 324, -34, 3324, 54];
        let cloned = arr3;
        exchange_sort(&mut arr3);
        assert!(is_sorted(&arr3) && have_same_elements(&arr3, &cloned));
    }
}
