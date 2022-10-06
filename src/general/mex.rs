use std::collections::BTreeSet;

// Find minimum excluded number from a set of given numbers using a set
// NOTE: Don't remove allow, else clippy error: function not used
#[allow(dead_code)]
fn mex_using_set(arr: &[i64]) -> i64 {
    let mut s: BTreeSet<i64> = BTreeSet::new();
    for i in 0..arr.len() + 1 {
        s.insert(i as i64);
    }
    for x in arr {
        s.remove(x);
    }
    return *s.first().unwrap();
}
// NOTE: Don't remove allow, else clippy error: function not used
#[allow(dead_code)]
fn mex_using_sort(arr: &[i64]) -> i64 {
    let mut arr = arr.to_vec();
    arr.sort();
    let mut mex = 0;
    for x in arr {
        if x == mex {
            mex += 1;
        }
    }
    mex
}

#[cfg(test)]
mod tests {
    use super::*;
    struct MexTests {
        test_arrays: Vec<Vec<i64>>,
        outputs: Vec<i64>,
    }
    impl MexTests {
        fn new() -> Self {
            return Self {
                test_arrays: vec![
                    vec![-1, 0, 1, 2, 3],
                    vec![-100, 0, 1, 2, 3, 5],
                    vec![-1000000, 0, 1, 2, 5],
                    vec![2, 0, 1, 2, 4],
                    vec![1, 2, 3, 0, 4],
                    vec![0, 1, 5, 2, 4, 3],
                    vec![0, 1, 2, 3, 4, 5, 6],
                    vec![0, 1, 2, 3, 4, 5, 6, 7],
                    vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
                ],
                outputs: vec![4, 4, 3, 3, 5, 6, 7, 8, 9],
            };
        }
        fn test_function(&self, f: fn(&[i64]) -> i64) {
            for (nums, output) in self.test_arrays.iter().zip(self.outputs.iter()) {
                assert_eq!(f(nums), *output);
            }
        }
    }
    #[test]
    fn test_mex_using_set() {
        let tests = MexTests::new();
        mex_using_set(&[1, 23, 3]);
        tests.test_function(mex_using_set);
    }
    #[test]
    fn test_mex_using_sort() {
        let tests = MexTests::new();
        tests.test_function(mex_using_sort);
    }
}
// Find minimum excluded number from a set of given numbers using sort
