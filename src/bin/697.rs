use std::collections::{BTreeSet, HashMap};

pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    let mut first_idx_map = HashMap::<i32, usize>::new();
    let mut occur_map = HashMap::<i32, i32>::new();
    let mut len_map = HashMap::<i32, i32>::new();

    for (idx, val) in nums.iter().enumerate() {
        first_idx_map.entry(*val).or_insert(idx);
        occur_map
            .entry(*val)
            .and_modify(|occur| *occur += 1)
            .or_insert(1);

        let first_idx = first_idx_map.get(val).unwrap();
        let len = idx - *first_idx + 1;
        len_map.insert(*val, len as i32);
    }

    let occur_set = occur_map
        .into_iter()
        .map(|(v, o)| (o, v))
        .collect::<BTreeSet<(i32, i32)>>();

    let (max_occur, _) = *occur_set.iter().last().unwrap();

    let max_occur_rel_vals = occur_set
        .into_iter()
        .filter(|(o, v)| *o == max_occur)
        .map(|(o, v)| v)
        .collect::<Vec<i32>>();

    let mut res = i32::MAX;
    for val in max_occur_rel_vals.into_iter() {
        res = res.min(*len_map.get(&val).unwrap());
    }

    res
}
fn main() {
    println!("{}", find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2]));
}
