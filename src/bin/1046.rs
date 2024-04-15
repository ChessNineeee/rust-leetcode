use std::collections::BTreeSet;

fn remove_heaviest_stones(stones: &mut BTreeSet<(i32, usize)>) {
    let (s1, idx1) = stones.pop_last().unwrap();
    let (s2, idx2) = stones.pop_last().unwrap();

    if s1 == s2 {
        return;
    }

    if s1 > s2 {
        stones.insert((s1 - s2, idx1));
        return;
    }

    stones.insert((s2 - s1, idx2));
    return;
}

pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut set = BTreeSet::new();
    for (idx, val) in stones.into_iter().enumerate() {
        set.insert((val, idx));
    }

    loop {
        if set.len() <= 1 {
            break;
        }

        remove_heaviest_stones(&mut set);
    }

    match set.pop_last() {
        None => 0,
        Some((val, _)) => val,
    }
}
fn main() {
    println!("{}", last_stone_weight(vec![2, 7, 4, 1, 8, 1]));
}
