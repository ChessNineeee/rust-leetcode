use std::collections::{BTreeMap, BTreeSet};

pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    let map: BTreeMap<String, usize> = list1
        .into_iter()
        .enumerate()
        .map(|(idx, val)| (val, idx))
        .collect();

    let mut set = BTreeSet::new();
    for (idx, val) in list2.into_iter().enumerate() {
        if !map.contains_key(&val) {
            continue;
        }

        let idx_sum = idx + *map.get(&val).unwrap();
        set.insert((idx_sum, val));
    }

    let (smallest_val, _) = set.first().unwrap();
    let smallest_val = *smallest_val;

    let res: Vec<String> = set
        .into_iter()
        .filter(|(idx, _)| *idx == smallest_val)
        .map(|(_, val)| val)
        .collect();

    res
}
fn main() {
    println!(
        "{:?}",
        find_restaurant(
            vec!["sad".to_string(), "happy".to_string(), "good".to_string()],
            vec!["happy".to_string(), "sad".to_string(), "good".to_string(),]
        )
    )
}
