fn get_sum(p_sum: &Vec<i32>, idx: usize) -> (i32, i32) {
    let mut left_side = 0;
    let mut right_side = 0;

    let last_idx = p_sum.len() - 1;

    match idx {
        0 => {
            right_side = *p_sum.last().unwrap() - *p_sum.first().unwrap();
            (left_side, right_side)
        }
        v if v == last_idx => {
            left_side = *p_sum.iter().rev().skip(1).next().unwrap();
            (left_side, right_side)
        }
        default if default < last_idx => {
            left_side = *p_sum.iter().nth(idx - 1).unwrap();
            right_side = *p_sum.last().unwrap() - *p_sum.iter().nth(idx).unwrap();
            (left_side, right_side)
        }
        _ => {
            unreachable!("{}", idx)
        }
    }
}

pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let prefix_sum: Vec<i32> = nums
        .into_iter()
        .scan(0, |sum, x| {
            *sum += x;
            Some(*sum)
        })
        .collect();

    for (idx, _) in prefix_sum.iter().enumerate() {
        let (left, right) = get_sum(&prefix_sum, idx);
        if left == right {
            return idx as i32;
        }
    }

    -1
}
fn main() {
    println!("{}", pivot_index(vec![1, 7, 3, 6, 5, 6]))
}
