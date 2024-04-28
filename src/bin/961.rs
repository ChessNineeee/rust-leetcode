pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
    for idx in 1..nums.len() {
        if nums[idx] == nums[idx - 1] {
            return nums[idx];
        }

        if idx < 2 {
            continue;
        }
        if nums[idx] == nums[idx - 2] {
            return nums[idx];
        }

        if idx < 3 {
            continue;
        }
        if nums[idx] == nums[idx - 3] {
            return nums[idx];
        }
    }

    unreachable!("never")
}
fn main() {
    println!("{}", repeated_n_times(vec![2, 1, 2, 5, 3, 2]));
}
