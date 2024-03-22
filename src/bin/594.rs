use std::collections::HashMap;

pub fn find_lhs(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for it in nums.into_iter() {
        map.entry(it).and_modify(|count| *count += 1).or_insert(1);
    }

    let mut res = 0;

    for (k, v) in map.iter() {
        let left = *k - 1;
        if !map.contains_key(&left) {
            continue;
        }

        let t = *v + *map.get(&left).unwrap();
        res = res.max(t);
    }

    res as i32
}
fn main() {
    println!("{}", find_lhs(vec![1,3,2,2,5,2,3,7]))
}
