use std::collections::{BTreeSet, HashMap};

fn gcd(x: i32, y: i32) -> i32 {
    if x == 0 {
        y
    } else {
        gcd(y % x, x)
    }
}

pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
    let set = deck
        .into_iter()
        .fold(HashMap::new(), |mut map, val| {
            map.entry(val).and_modify(|v| *v += 1).or_insert(1);
            map
        })
        .into_iter()
        .map(|(_, v)| v)
        .collect::<BTreeSet<i32>>();

    set.into_iter().fold(0, |acc, v| gcd(acc, v)) >= 2
}

fn main() {
    println!("{}", has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 2, 2, 2, 1]))
}
