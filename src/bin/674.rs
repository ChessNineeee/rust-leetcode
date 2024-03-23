pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
    let mut res = 1;
    let mut left_closed = 0;
    let mut right_closed = 1;
    loop {
        if right_closed >= nums.len() {
            break;
        }

        let cur = nums.get(right_closed).unwrap();
        let prev_idx = right_closed - 1;
        let prev = nums.get(prev_idx).unwrap();

        if *cur > *prev {
            res = res.max((right_closed - left_closed + 1) as i32);
            right_closed += 1;
            continue;
        }

        left_closed = right_closed;
        right_closed += 1;
    }
    res
}
fn main() {
    println!("{}", find_length_of_lcis(vec![2, 2, 2, 2, 2]))
}
