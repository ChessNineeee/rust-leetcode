pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
    let mut last_num = 0;
    // nums % 5 == (nums%5)%5
    nums.into_iter()
        .map(|digit| {
            let cur_num = (last_num << 1) + digit;
            last_num = cur_num % 5;
            cur_num % 5 == 0
        })
        .collect()
}
fn main() {
    println!("{:?}", prefixes_div_by5(vec![1, 1, 0, 0, 0, 1, 0, 0, 1]))
}
