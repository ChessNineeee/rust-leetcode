pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut count = 0;

    for it in nums.iter() {
        if *it == 0 {
            res = res.max(count);
            count = 0;
            continue;
        }
        count += 1;
    }

    res = res.max(count);

    return res;
}
fn main() {
    println!("{}", find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]))
}
