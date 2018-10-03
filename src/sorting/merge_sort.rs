fn merge<T: Copy + PartialOrd>(x1: &[T], x2: &[T], y: &mut [T]) {
	assert_eq!(x1.len() + x2.len(), y.len());
	let mut i = 0;
	let mut j = 0;
	let mut k = 0;
	while i < x1.len() && j < x2.len() {
		if x1[i] < x2[j] {
			y[k] = x1[i];
			k += 1;
			i += 1;
		} else {
			y[k] = x2[j];
			k += 1;
			j += 1;
		}
	}
	if i < x1.len() {
		y[k..].copy_from_slice(&x1[i..]);
	}
	if j < x2.len() {
		y[k..].copy_from_slice(&x2[j..]);
	}
}

pub fn merge_sort<T: Copy + Ord>(arr: &mut [T]) {
    let n = arr.len();
	let m = n / 2;
 
	if n <= 1 {
		return;
	}
 
	merge_sort(&mut arr[0..m]);
	merge_sort(&mut arr[m..n]);
 
	let mut y: Vec<T> = arr.to_vec();
 
	merge(&arr[0..m], &arr[m..n], &mut y[..]);
 
	arr.copy_from_slice(&y);
}

#[cfg(test)]
mod tests {
    #[test]
    fn merge_test() {
        use sorting::merge_sort::*;

        let mut data = Vec::<u8>::new();
        merge_sort(data.as_mut_slice());
        assert_eq!(data, vec![]);

        let mut data = vec!["a"];
        merge_sort(data.as_mut_slice());
        assert_eq!(data, vec!["a"]);

        let mut data = vec!["a", "b", "c"];
        merge_sort(data.as_mut_slice());
        assert_eq!(data, vec!["a", "b", "c"]);

        let mut data = vec!["d", "a", "c", "b"];
        merge_sort(data.as_mut_slice());
        assert_eq!(data, vec!["a", "b", "c", "d"]);

        let mut data = vec![5,3,8,9,15,1,4,4,0];
        merge_sort(data.as_mut_slice());
        assert_eq!(data, vec![0,1,3,4,4,5,8,9,15]);
    }
}
