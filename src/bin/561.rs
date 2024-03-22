pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    nums.iter().step_by(2).sum()
}
fn main() {
    println!("{}", array_pair_sum(vec![6,2,6,5,1,2]))
}
