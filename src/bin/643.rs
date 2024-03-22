fn get_v(v: &Vec<i32>, idx: usize) -> i32 {
    if idx == 0 {
        return 0;
    }

    v[idx - 1]
}
pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut sum = nums[0];
    let mut nums = nums;
    for i in 1..nums.len() {
        sum = sum + nums[i];
        nums[i] = sum;
    }
    let mut left_closed: usize = 0;
    let mut right_open: usize = k as usize;
    let mut res = (get_v(&nums, right_open) - get_v(&nums, left_closed)) as f64 / k as f64;

    left_closed += 1;
    right_open += 1;

    loop {
        if right_open > nums.len() {
            break;
        }

        res = res.max((get_v(&nums, right_open) - get_v(&nums, left_closed)) as f64 / k as f64);
        left_closed += 1;
        right_open += 1;
    }

    res
}
fn main() {
    println!("{}", find_max_average(vec![1, 12, -5, -6, 50, 3], 4));
}
