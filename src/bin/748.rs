use std::collections::HashMap;

fn submap(map1: &HashMap<char, i32>, map2: &HashMap<char, i32>) -> bool {
    for (k, v) in map1.iter() {
        if !map2.contains_key(k) {
            return false;
        }

        let v2 = map2.get(k).unwrap();
        if v > v2 {
            return false;
        }
    }
    true
}

pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
    let plate_char_occ = license_plate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .fold(HashMap::new(), |mut map, c| {
            map.entry(c).and_modify(|occ| *occ += 1).or_insert(0);
            map
        });

    let mut shortest_len = usize::MAX;
    let mut shortest_idx = usize::MAX;

    for (idx, word) in words.iter().enumerate() {
        let word_char_occ = word.chars().fold(HashMap::new(), |mut map, c| {
            map.entry(c).and_modify(|occ| *occ += 1).or_insert(0);
            map
        });

        if !submap(&plate_char_occ, &word_char_occ) {
            continue;
        }

        if word.len() >= shortest_len {
            continue;
        }

        shortest_len = word.len();
        shortest_idx = idx;
    }

    words[shortest_idx].clone()
}
fn main() {
    println!(
        "{}",
        shortest_completing_word(
            "1s3 456".to_string(),
            vec![
                "looks".to_string(),
                "pest".to_string(),
                "stew".to_string(),
                "show".to_string()
            ]
        )
    );
}
