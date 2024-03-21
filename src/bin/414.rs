use std::collections::BTreeSet;

fn set_insert(set: &mut BTreeSet<i32>, num: i32) {
    set.insert(num);
    if set.len() > 3 {
        set.pop_first();
    }
    return;
}

fn get_third_max(set: &BTreeSet<i32>) -> i32 {
    if set.len() < 3 {
        return *set.last().unwrap();
    }

    return *set.first().unwrap();
}

pub fn third_max(nums: Vec<i32>) -> i32 {
    let mut set = BTreeSet::new();

    for it in nums.iter() {
        set_insert(&mut set, *it);
    }

    return get_third_max(&set);
}
fn main() {
    println!("{}", third_max(vec![1, 2, 2, 5, 3, 5]));
}
