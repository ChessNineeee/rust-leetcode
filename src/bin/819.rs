use std::collections::{BTreeSet, HashMap, HashSet};

pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
    let banned = banned.into_iter().collect::<HashSet<String>>();
    let set = paragraph
        .split(|c: char| c == ' ' || c.is_ascii_punctuation())
        .filter(|str| !str.is_empty())
        .map(|str| str.to_lowercase())
        .filter(|str| !banned.contains(str))
        .fold(HashMap::new(), |mut map, str| {
            map.entry(str).and_modify(|v| *v += 1).or_insert(1);
            map
        })
        .into_iter()
        .map(|(k, v)| (v, k))
        .collect::<BTreeSet<(i32, String)>>();

    let (_, res) = set.last().unwrap();
    res.clone()
}
fn main() {
    println!(
        "{}",
        most_common_word(
            "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
            vec!["hit".to_string()]
        )
    )
}
