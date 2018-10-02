 fn lis(x: Vec<i32>)-> Vec<i32> {
    let n = x.len();
    let mut m = vec![0; n];
    let mut p = vec![0; n];
    let mut l = 0;
    for i in 0..n {
        let mut lo = 1;
        let mut hi = l;
        while lo <= hi {
            let mut mid = (lo + hi) / 2;
            if x[m[mid]] <= x[i] {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        let mut new_l = lo;
        p[i] = m[new_l - 1];
        m[new_l] = i;
        if new_l > l {
            l = new_l;
        }
    }
    let mut o = vec![0; l];
    let mut k = m[l];
    for i in (0..l).rev() {
        o[i] = x[k];
        k    = p[k];
    }
    o
}
fn main() {
    let v = vec![0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15];
    let o = lis(v);
    println!("{:?}", o);
}
