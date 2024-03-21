use std::collections::{HashMap, VecDeque};

fn monotonic_q_insert(q: &mut VecDeque<i32>, num: i32) -> i32 {
    // return the element under the newly inserted one
    if q.is_empty() {
        q.push_front(num);
        return -1;
    }

    loop {
        if q.is_empty() {
            q.push_front(num);
            return -1;
        }

        let top = *q.front().unwrap();
        if top < num {
            q.pop_front();
            continue;
        }

        q.push_front(num);
        return top;
    }
}

// 1, 3, 5, 7, 9
fn next_greater_elements(nums: &Vec<i32>) -> HashMap<i32, i32> {
    let mut q = VecDeque::new();
    let mut res = HashMap::new();
    for it in nums.iter().rev() {
        res.insert(*it, monotonic_q_insert(&mut q, *it));
    }

    res
}
pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let next_g_elements = next_greater_elements(&nums2);

    let mut res = Vec::new();

    for it in nums1.iter() {
        res.push(*next_g_elements.get(it).unwrap());
    }
    res
}

fn main() {
    println!("{:?}", next_greater_element(vec![2, 4], vec![1, 2, 3, 4]))
}
