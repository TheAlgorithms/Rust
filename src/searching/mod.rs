use std::cmp;

pub fn binary_search<T>(item: T, arr: &[T]) -> i32
where
    T: cmp::PartialEq + cmp::PartialOrd + Sized,
{
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;

        if arr[mid] > item {
            right = mid - 1;
        } else if arr[mid] < item {
            left = mid + 1;
        } else {
            left = mid;
            break;
        }
    }

    if arr[left] != item {
        return -1;
    }

    left as i32
}

pub fn linear_search<T>(item: T, arr: &[T]) -> i32
where
    T: cmp::PartialEq,
{
    let length = arr.len();

    for i in 0..length {
        if item == arr[i] {
            return i as i32;
        }
    }

    return -1;
}


/// Shell Sort
/// The values to go by for a shell-sort. Note that the sequence determines the complexity.
pub trait ShellHs : Iterator<Item=usize> {
    /// Create a new ShellHs, for a vector of length n
    fn new(n: usize) -> Self;
}

/// Knuth's values: 1,4,13,40,121... up to n/3
#[derive(Copy, Clone, Debug)]
pub struct ShellKnuth {
    h : usize
}

impl Iterator for ShellKnuth {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.h /= 3;
        match self.h {
            0 => None,
            value => Some(value)
        }
    }
}

impl ShellHs for ShellKnuth {
    fn new(n: usize) -> ShellKnuth {
        let mut h = 4;
        while h*3 <= n {
            h = 3*h + 1;
        }

        ShellKnuth{h: h}
    }
}

fn insertion_sort_partial<T : Ord>(slice : &mut [T], start: usize, step: usize){
    for i in (start+step)..slice.len(){
        let mut curloc = i;
        while (curloc >= step) && slice[curloc] < slice[curloc-step] {
            slice.swap(curloc, curloc-step);
            curloc -= step;
        }
    }
}

/// Shell sort
pub fn shellsort<H : ShellHs, T : Ord>(slice : &mut [T]){
    let hs : H = ShellHs::new(slice.len());
    for h in hs {
        for k in 0..h {
            // our sublist is now [k, h+k, 2h+k,...]
            // We insertion sort it
            insertion_sort_partial(slice, k, h);
        }
    }
}

// another way of shell sort
fn shellsort2(mut array: &mut [i32])
{
    let mut c = array.len()/2;
    while c > 0
        {
            for s in 0..c
                {
                    insercion(&mut array, s, c);
                }
            c = c/2;
        }
}
fn insercion(mut array: &mut [i32], i: usize, g: usize)
{
    let x = i+g;
    let n = array.len();
    for i in x..n
        {
            let valor=array[i];
            let mut posicion=i;
            while posicion>=g && array[posicion-g]>valor
                {
                    array[posicion] = array[posicion-g];
                    posicion = posicion-g;
                }
            array[posicion]=valor;
        }
}

#[cfg(test)]
mod tests {
    #[test]
    fn linear() {
        use searching;
        let index = searching::linear_search("a", &vec!["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, 0);

        let mut index = searching::linear_search(4, &vec![1, 2, 3, 4]);
        assert_eq!(index, 3);

        index = searching::linear_search(3, &vec![1, 2, 3, 4]);
        assert_eq!(index, 2);

        index = searching::linear_search(2, &vec![1, 2, 3, 4]);
        assert_eq!(index, 1);

        index = searching::linear_search(1, &vec![1, 2, 3, 4]);
        assert_eq!(index, 0);

        index = searching::linear_search(5, &vec![1, 2, 3, 4]);
        assert_eq!(index, -1);
    }
}