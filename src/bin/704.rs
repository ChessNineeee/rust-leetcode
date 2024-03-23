pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left_closed = 0;
    let mut right_closed = (nums.len() - 1) as i32;

    loop {
        if left_closed > right_closed {
            break;
        }

        let mid = left_closed + (right_closed - left_closed) / 2;
        if nums[mid as usize] == target {
            return mid as i32;
        }

        if nums[mid as usize] > target {
            right_closed = mid - 1;
            continue;
        }

        left_closed = mid + 1;
    }

    -1
}
fn main() {
    println!("{}", search(vec![4], 1))
}
