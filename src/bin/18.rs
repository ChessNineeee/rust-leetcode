fn first_n_sum(nums: &Vec<i32>, start_idx: usize, n: usize) -> i64 {
    let mut res = 0;
    for idx in start_idx..start_idx + n {
        res += nums[idx] as i64;
    }
    res
}

fn last_n_sum(nums: &Vec<i32>, n: usize) -> i64 {
    let mut res = 0;
    let len = nums.len();
    for idx in len - n..len {
        res += nums[idx] as i64;
    }
    res
}

fn two_sum(nums: &Vec<i32>, target: i32, start_idx: usize) -> Vec<(i32, i32)> {
    println!("{}", target);
    let mut i = start_idx;
    let mut j = nums.len() - 1;
    let mut res = Vec::new();

    loop {
        if i == j {
            break;
        }

        if is_duplicate(nums, start_idx, i) {
            i += 1;
            continue;
        }

        let v_i = nums[i];
        let v_j = nums[j];
        let cur_sum = v_i + v_j;

        if cur_sum == target {
            res.push((v_i, v_j));
            i += 1;
            continue;
        }

        if cur_sum < target {
            i += 1;
            continue;
        }

        j -= 1;
        continue;
    }

    res
}

fn three_sum(nums: &Vec<i32>, target: i32, start_idx: usize) -> Vec<(i32, i32, i32)> {
    let mut i = start_idx;
    let mut res = Vec::new();

    let len = nums.len();

    loop {
        if i + 2 >= len {
            break;
        }

        if is_duplicate(nums, start_idx, i) {
            i += 1;
            continue;
        }

        let v_i = nums[i];

        if v_i as i64 + first_n_sum(nums, i + 1, 2) > target as i64 {
            break;
        }

        if target as i64 > v_i as i64 + last_n_sum(nums, 2) {
            i += 1;
            continue;
        }

        let new_target = target.checked_sub(v_i);
        if new_target.is_none() {
            break;
        }
        let new_start_idx = i + 1;

        let two_sum_res = two_sum(nums, new_target.unwrap(), new_start_idx);

        for (first_num, second_num) in two_sum_res.into_iter() {
            res.push((v_i, first_num, second_num));
        }

        i += 1;
    }

    res
}

fn is_duplicate(nums: &Vec<i32>, greater_than: usize, idx: usize) -> bool {
    if idx <= greater_than {
        return false;
    }

    return nums[idx] == nums[idx - 1];
}

pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();

    let mut res = Vec::new();

    let mut i = 0;

    let len = nums.len();

    loop {
        if i + 3 >= len {
            break;
        }

        if is_duplicate(&nums, 0, i) {
            i += 1;
            continue;
        }

        let v_i = nums[i];
        if v_i as i64 + first_n_sum(&nums, i + 1, 3) > target as i64 {
            break;
        }

        if target as i64 > v_i as i64 + last_n_sum(&nums, 3) {
            i += 1;
            continue;
        }
        let new_target = target.checked_sub(v_i);
        if new_target.is_none() {
            break;
        }
        let new_start_idx = i + 1;

        let three_sum_res = three_sum(&nums, new_target.unwrap(), new_start_idx);

        for (_1, _2, _3) in three_sum_res.into_iter() {
            res.push(vec![v_i, _1, _2, _3]);
        }

        i += 1;
    }

    res
}

fn main() {
    println!(
        "{:?}",
        four_sum(
            vec![1000000000, 1000000000, 1000000000, 1000000000],
            -294967296
        )
    );
}
