use std::collections::HashMap;

fn m_insert(m: &mut HashMap<i32, usize>, k: &i32) {
    m.entry(*k).and_modify(|v| *v += 1).or_insert(0);
}

pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut map = HashMap::new();
    let mut map2 = HashMap::new();

    for it in nums1.iter() {
        m_insert(&mut map, it);
    }

    for it in nums2.iter() {
        m_insert(&mut map2, it);

        if !map.contains_key(it) {
            continue;
        }

        if *map.get(it).unwrap() < *map2.get(it).unwrap() {
            continue;
        }

        res.push(*it);
    }

    return res;
}
fn main() {
    let nums1 = vec![4, 9, 5];
    let nums2 = vec![9, 4, 9, 8, 4];

    println!("{:?}", intersect(nums1, nums2));
}
