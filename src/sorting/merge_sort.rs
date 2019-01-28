pub fn merge_sort<T:Ord + Clone>(arr: &mut [T]){
    if arr.len() > 1 {
        let n = arr.len();
        let middle = (n + 1) / 2;
        merge_sort(&mut arr[0..middle]);
        merge_sort(&mut arr[middle..n]);
        let mut buffer:Vec<T> = Vec::new();
        {
            let mut i: usize = 0;
            let mut j: usize = middle;
            while i < middle && j < n {
                if arr[i] < arr[j] {
                    buffer.push(arr[i].clone());
                    i = i + 1;
                } else {
                    buffer.push(arr[j].clone());
                    j = j + 1;
                }
            }
            while i < middle {
                buffer.push(arr[i].clone());
                i = i + 1;
            }
            while j < n {
                buffer.push(arr[j].clone());
                j = j + 1;
            }
        }
        for idx in 0..arr.len(){
            arr[idx] = buffer[idx].clone();
        }
    }
}
