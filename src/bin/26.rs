pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut res = 1;
    let mut left = 1;
    let mut right = 1;

    let len = nums.len();

    loop {
        if right >= len {
            break;
        }

        let prev = right - 1;
        if nums[right] == nums[prev] {
            right += 1;
            continue;
        }

        nums[left] = nums[right];
        left += 1;
        right += 1;
        res += 1;
    }
    res
}
fn main() {
    println!("{}", remove_duplicates(&mut vec![1, 1, 2]));
}
