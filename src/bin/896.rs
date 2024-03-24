pub fn is_monotonic(nums: Vec<i32>) -> bool {
    let mut desc = true;
    let mut asc = true;
    for i in 0..nums.len() {
        if i == 0 {
            continue;
        }

        let prev_idx = i - 1;
        if nums[i] > nums[prev_idx] {
            desc = false;
        } else if nums[i] < nums[prev_idx] {
            asc = false;
        } else {
            continue;
        }
    }
    desc || asc
}
fn main() {
    println!("{}", is_monotonic(vec![1, 3, 2]))
}
