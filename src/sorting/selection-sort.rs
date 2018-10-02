fn selection_sort(array: &mut [i32]) {
    let mut min;
    for i in 0..array.len() {
        min = i;
        for j in (i+1)..array.len() {
            if array[j] < array[min] {
                min = j;
            }
        }
        let tmp = array[i];
        array[i] = array[min];
        array[min] = tmp;
    }
}
fn main() {
    let mut values = [ 5, 8, 4, 1, 7, 2, 3, 6 ];
    println!("Elements of the array before sorting {:?}", values);
    selection_sort(&mut values);
    println!("Elements of the array after sorting {:?}", values);
}
