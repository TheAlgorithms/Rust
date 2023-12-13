/*

    Moore's voting algorithm finds out the strictly majority-occurring element
    without using extra space
    and O(n) + O(n) time complexity

    It is built on the intuition that a strictly major element will always have a net occurrence as 1.
    Say, array given: 9 1 8 1 1
    Here, the algorithm will work as:

    (for finding element present >(n/2) times)
    (assumed: all elements are >0)

    Initialisation: ele=0, cnt=0
    Loop beings.

    loop 1: arr[0]=9
    ele = 9
    cnt=1 (since cnt = 0, cnt increments to 1 and ele = 9)

    loop 2: arr[1]=1
    ele = 9
    cnt= 0 (since in this turn of the loop, the array[i] != ele, cnt decrements by 1)

    loop 3: arr[2]=8
    ele = 8
    cnt=1 (since cnt = 0, cnt increments to 1 and ele = 8)

    loop 4: arr[3]=1
    ele = 8
    cnt= 0 (since in this turn of the loop, the array[i] != ele, cnt decrements by 1)

    loop 5: arr[4]=1
    ele = 9
    cnt=1 (since cnt = 0, cnt increments to 1 and ele = 1)

    Now, this ele should be the majority element if there's any
    To check, a quick O(n) loop is run to check if the count of ele is >(n/2), n being the length of the array

    -1 is returned when no such element is found.

*/

pub fn moore_voting(arr: &[i32]) -> i32 {
    let n = arr.len();
    let mut cnt = 0; // initializing cnt
    let mut ele = 0; // initializing ele

    arr.iter().for_each(|&item| {
        if cnt == 0 {
            cnt = 1;
            ele = item;
        } else if item == ele {
            cnt += 1;
        } else {
            cnt -= 1;
        }
    });

    let cnt_check = arr.iter().filter(|&&x| x == ele).count();

    if cnt_check > (n / 2) {
        ele
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moore_voting() {
        let arr1: Vec<i32> = vec![9, 1, 8, 1, 1];
        assert!(moore_voting(&arr1) == 1);
        let arr2: Vec<i32> = vec![1, 2, 3, 4];
        assert!(moore_voting(&arr2) == -1);
    }
}
