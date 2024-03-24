use std::collections::BTreeSet;

pub fn dominant_index(nums: Vec<i32>) -> i32 {
    let mut set = BTreeSet::new();

    for (idx, val) in nums.into_iter().enumerate() {
        set.insert((val, idx));

        if set.len() > 2 {
            set.pop_first();
        }
    }

    let (max_v, max_v_idx) = *set.last().unwrap();
    let (second_max_v, _) = *set.first().unwrap();

    if second_max_v * 2 <= max_v {
        max_v_idx as i32
    } else {
        -1
    }
}
fn main() {
    println!("{}", dominant_index(vec![1, 2, 3, 4]))
}
