use std::collections::HashSet;

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut res = HashSet::new();
    let mut set = HashSet::new();
    for it in nums1.iter() {
        set.insert(*it);
    }

    for it in nums2.iter() {
        if set.contains(it) {
            res.insert(*it);
        }
    }
    return res.into_iter().collect();
}
fn main() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];

    println!("{:?}", intersection(nums1, nums2));
}
