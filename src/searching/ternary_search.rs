fn main() {
    let nums = vec![1, 3, 5, 7, 9]; //Change the values of array here
    let ele = 5; #Element to be searched
    println!("Sample input list: {:?}", nums);
    println!("Searched for {} and found index {}", ele, ternary_search(&nums, 0, nums.len(), ele));
}
fn ternary_search(nums: &Vec<i64>, left: usize, right: usize, x: i64) -> i64 {
    if left <= right {
        let intvl = (right - left) / 3;
        let leftmid = left + intvl;
        let rightmid = leftmid + intvl;
       if nums[leftmid] == x {
           return leftmid as i64
       }
       if nums[rightmid] == x {
           return rightmid as i64;
       }
       if x < nums[leftmid] {
           return ternary_search(&nums, left, leftmid, x);
       }
       else if x > nums[leftmid] && x < nums[rightmid] {
           return ternary_search(&nums, leftmid, rightmid, x);
       }
       else {
           return ternary_search(&nums, rightmid, right, x);
       }
   }
   -1
}