fn two_sum(nums: &Vec<i32>, sum: i32, start_idx: usize) -> Vec<(i32, i32)> {
    let mut j = start_idx;
    let mut k = nums.len() - 1;

    let mut res = Vec::new();

    loop {
        if j == k {
            break;
        }

        if is_duplicate(nums, start_idx, j) {
            j += 1;
            continue;
        }

        let v_j = nums[j];
        let v_k = nums[k];
        let cur_sum = v_j + v_k;

        if cur_sum == sum {
            res.push((v_j, v_k));
            j += 1;
            continue;
        }

        if cur_sum > sum {
            k -= 1;
            continue;
        }

        j += 1;
        continue;
    }
    res
}

fn is_duplicate(nums: &Vec<i32>, bigger_than: usize, idx: usize) -> bool {
    if idx <= bigger_than {
        return false;
    }

    let v_idx = nums[idx];
    let v_idx_prev = nums[idx - 1];

    return v_idx == v_idx_prev;
}

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();

    let mut res = Vec::new();
    let mut i = 0;

    let len = nums.len();

    loop {
        if i >= len - 1 {
            break;
        }

        if is_duplicate(&nums, 0, i) {
            i += 1;
            continue;
        }

        let v_i = nums[i];
        let start_idx = i + 1;
        let sum = -1 * v_i;
        let two_sum_res = two_sum(&nums, sum, start_idx);

        for two_sum in two_sum_res.into_iter() {
            let mut tmp_res = vec![v_i];
            tmp_res.push(two_sum.0);
            tmp_res.push(two_sum.1);

            res.push(tmp_res);
        }

        i += 1;
    }
    res
}
fn main() {
    println!("{:?}", three_sum(vec![-1, 0, 1, 2, -1, -4]));
}
