fn selection_sort(list: &mut [i64]) {
  for i in 0..list.len() {
    let mut small = i;
    for j in (i + 1)..list.len() {
      if list[j] < list[small] {
        small = j;
      }
    }
    list.swap(small, i);
  }
}
