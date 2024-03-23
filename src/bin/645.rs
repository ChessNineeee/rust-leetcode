use std::collections::HashSet;

pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len() as i32;
    let sum = (1 + len) * len / 2;
    let n_sum = nums.iter().sum::<i32>();
    let unique_sum = nums
        .into_iter()
        .collect::<HashSet<i32>>()
        .into_iter()
        .sum::<i32>();
    return vec![n_sum - unique_sum, sum - unique_sum];
}
fn main() {
    println!("{:?}", find_error_nums(vec![1, 1]))
}
