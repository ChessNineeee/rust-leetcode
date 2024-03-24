pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut idx_even_open = 0;
    let mut idx_next_even = 1;
    loop {
        if idx_next_even >= nums.len() {
            break;
        }

        if idx_even_open > idx_next_even {
            idx_next_even = idx_even_open + 1;
            continue;
        }

        if nums[idx_next_even] & 1 == 1 {
            idx_next_even += 1;
            continue;
        }

        if nums[idx_even_open] & 1 == 0 {
            idx_even_open += 1;
            continue;
        }

        nums.swap(idx_even_open, idx_next_even);
    }

    nums
}
fn main() {
    println!("{:?}", sort_array_by_parity(vec![0]))
}
