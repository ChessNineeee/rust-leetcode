fn two_sum_closest(nums: &Vec<i32>, target: i32, start_idx: usize) -> (i32, i32) {
    let mut j = start_idx;
    let mut k = nums.len() - 1;
    let mut diff = i32::MAX;
    let mut sum = i32::MIN;

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
        let d = (cur_sum - target).abs();

        if d < diff {
            diff = d;
            sum = cur_sum;
        }

        if cur_sum == target {
            break;
        }

        if cur_sum < target {
            j += 1;
            continue;
        }

        k -= 1;
    }

    (diff, sum)
}

fn is_duplicate(nums: &Vec<i32>, greater_than: usize, idx: usize) -> bool {
    if idx <= greater_than {
        return false;
    }

    let v_idx = nums[idx];
    let v_idx_prev = nums[idx - 1];

    return v_idx == v_idx_prev;
}

pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let len = nums.len();
    let mut i = 0;
    let mut diff = i32::MAX;
    let mut res = i32::MIN;
    loop {
        if i == len - 1 {
            break;
        }

        if is_duplicate(&nums, 0, i) {
            i += 1;
            continue;
        }

        let v_i = nums[i];
        let start_idx = i + 1;
        let (dif, sum) = two_sum_closest(&nums, target - v_i, start_idx);

        if dif < diff {
            diff = dif;
            res = sum + v_i;
        }

        i += 1;
    }

    res
}

fn main() {
    println!("{}", three_sum_closest(vec![-1, 2, 1, -4], 1));
}
