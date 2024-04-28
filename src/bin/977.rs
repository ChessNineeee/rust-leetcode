pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; nums.len()];
    let mut c_idx = 0;

    let mut a_idx = nums.len();
    for (idx, val) in nums.iter().enumerate() {
        if *val >= 0 {
            a_idx = idx;
            break;
        }
    }
    let mut b_idx = a_idx - 1;

    loop {
        if c_idx == res.len() {
            break;
        }

        if b_idx == usize::MAX {
            res[c_idx] = nums[a_idx].checked_pow(2).unwrap();
            a_idx += 1;
            c_idx += 1;
            continue;
        }

        if a_idx == nums.len() {
            res[c_idx] = nums[b_idx].checked_pow(2).unwrap();
            c_idx += 1;
            b_idx = b_idx.checked_sub(1).or_else(|| Some(usize::MAX)).unwrap();
            continue;
        }

        if nums[b_idx].abs() <= nums[a_idx] {
            res[c_idx] = nums[b_idx].checked_pow(2).unwrap();
            c_idx += 1;
            b_idx = b_idx.checked_sub(1).or_else(|| Some(usize::MAX)).unwrap();
            continue;
        }

        res[c_idx] = nums[a_idx].checked_pow(2).unwrap();
        c_idx += 1;
        a_idx += 1;
    }
    res
}
fn main() {
    println!("{:?}", sorted_squares(vec![-7, -3, 2, 3, 11]));
}
