pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    for j in (0..nums.len()).rev() {
        if j < 2 {
            break;
        }

        let line1 = nums[j - 2];
        let line2 = nums[j - 1];
        let line3 = nums[j];

        if line1 + line2 <= line3 {
            continue;
        }

        return line1 + line2 + line3;
    }
    0
}

fn main() {
    println!("{}", largest_perimeter(vec![1, 2, 1, 10]));
}
