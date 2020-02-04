use std::cmp;

#[allow(dead_code)]
pub fn merge_sort<T>(arr: &[T]) -> Vec<T>
where
    T: cmp::PartialEq + cmp::PartialOrd + Clone,
{
    if arr.len() <= 1 {
        return arr.iter().cloned().collect();
    }

    let mut result = Vec::with_capacity(arr.len());

    let middle = arr.len() / 2;
    let (left, right) = arr.split_at(middle);
    let left = merge_sort(left);
    let right = merge_sort(right);

    let mut l = 0;
    let mut r = 0;
    while r < right.len() || l < left.len() {
        let left = left.get(l);
        let right = right.get(r);

        if left.is_none() {
            result.push(right.unwrap().clone());
            r += 1;
        } else if right.is_none() {
            result.push(left.unwrap().clone());
            l += 1;
        } else if right.unwrap() < left.unwrap() {
            result.push(right.unwrap().clone());
            r += 1;
        } else {
            result.push(left.unwrap().clone());
            l += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let res = merge_sort(&Vec::<u8>::new());
        assert_eq!(res, vec![]);
    }

    #[test]
    fn one_element() {
        let res = merge_sort(&vec![1]);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn already_sorted() {
        let res = merge_sort(&vec![1, 2, 3, 4]);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn not_sorted() {
        let res = merge_sort(&vec![2, 1, 4, 3]);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn repeated_elements() {
        let res = merge_sort(&vec![542, 542, 542, 542]);
        assert_eq!(res, vec![542, 542, 542, 542]);
    }
}
