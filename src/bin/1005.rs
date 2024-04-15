use std::collections::BTreeSet;
fn smallest_k_insert(s_k: &mut BTreeSet<(i32, usize)>, num: (i32, usize), k: i32) {
    s_k.insert(num);
    if s_k.len() > k as usize {
        s_k.pop_last();
    }
}

pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
    let mut smallest_k = BTreeSet::new();
    let mut sum = 0;
    for (idx, num) in nums.into_iter().enumerate() {
        smallest_k_insert(&mut smallest_k, (num, idx), k);
        sum += num;
    }

    let mut k = k;
    loop {
        if k == 0 {
            break;
        }

        let (smallest_v, idx) = smallest_k.pop_first().unwrap();
        if smallest_v < 0 {
            k -= 1;
            sum += smallest_v.abs() * 2;
            smallest_k.insert((smallest_v.abs(), idx));
            continue;
        }

        if smallest_v == 0 {
            k = 0;
            continue;
        }

        if k % 2 == 0 {
            k = 0;
            continue;
        }

        sum -= 2 * smallest_v;
        k = 0;
    }
    sum
}

fn main() {
    println!(
        "{}",
        largest_sum_after_k_negations(vec![-2, 5, 0, 2, -2], 3)
    )
}
