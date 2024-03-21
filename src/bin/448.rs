pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.insert(0, 0);

    let n = nums.len();
    for i in 1..n {
        let j = nums[i].abs() as usize;
        if nums[j] < 0 {
            continue;
        }

        nums[j] = nums[j] * -1;
    }

    nums.into_iter()
        .enumerate()
        .filter_map(|(i, v)| -> Option<i32> {
            if v > 0 {
                Some(i as i32)
            } else {
                None
            }
        })
        .collect()
}
fn main() {
    println!("{:?}", find_disappeared_numbers(vec![1, 1]));
}
