pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut count = 0;
    let len = nums.len();

    for i in 0..len {
        if nums[i] == 0 {
            count += 1;
            continue;
        }

        nums[i - count] = nums[i];
    }

    let start = nums.len() - count;
    for i in start..nums.len() {
        nums[i] = 0;
    }
}
fn main() {
    let mut v = vec![1, 0, 0, 3, 12];

    move_zeroes(&mut v);

    println!("{:?}", v);
}
