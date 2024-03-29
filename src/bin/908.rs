use std::collections::BTreeSet;

pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
    if nums.len() < 2 {
        return 0;
    }

    let nums: BTreeSet<i32> = nums.into_iter().collect();

    let max_num = *nums.last().unwrap();
    let min_num = *nums.first().unwrap();

    let diff = max_num - min_num;
    if diff > k * 2 {
        return diff - 2 * k;
    }
    0
}
fn main() {
    println!("{}", smallest_range_i(vec![1, 3, 6], 3))
}
