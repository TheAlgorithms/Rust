use std::io;

struct Solution {
    solver: fn(&Vec<i32>) -> i32,
}

impl Solution {
    fn new() -> Solution {
        Solution {
            solver: Solution::moore_voting,
        }
    }

    fn moore_voting(arr: &Vec<i32>) -> i32 {
        let n = arr.len();
        let mut cnt = 0; // initializing cnt
        let mut ele = 0; // initializing ele

        for i in 0..n {
            if cnt == 0 {
                cnt = 1;
                ele = arr[i];
            } else if arr[i] == ele {
                cnt += 1;
            } else {
                cnt -= 1;
            }
        }

        let cnt_check = arr.iter().filter(|&&x| x == ele).count();

        if cnt_check > (n / 2) {
            ele
        } else {
            -1
        }
    }

    fn caller_func(&self, arr: &Vec<i32>) -> i32 {
        (self.solver)(arr)
    }
}

fn main() {

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
    */
    
    let mut input = String::new();
    println!("Number of elements: ");
    io::stdin().read_line(&mut input).expect("");
    let n: usize = input.trim().parse().expect("");

    let mut arr = vec![0; n];

    println!("Enter array elements: ");
    input.clear();
    io::stdin().read_line(&mut input).expect("");
    let elements: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect(""))
        .collect();
    arr.copy_from_slice(&elements);

    let soln = Solution::new();
    let solution = soln.caller_func(&arr);

    if solution == -1 {
        println!("No element is present >(n/2) times.");
    } else {
        println!("{} is present >(n/2) times.", solution);
    }
}
