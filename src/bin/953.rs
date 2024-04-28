use std::collections::HashMap;

fn is_s1_bigger_than_s2(order: &HashMap<char, usize>, s1: &String, s2: &String) -> bool {
    let len1 = s1.len();
    let len2 = s2.len();
    let mut idx = 0;
    loop {
        assert!(idx <= len1 && idx <= len2);
        if idx == len1 && idx == len2 {
            return true;
        }

        if idx == len1 {
            return false;
        }

        if idx == len2 {
            return true;
        }

        let c1 = s1.chars().nth(idx).unwrap();
        let c2 = s2.chars().nth(idx).unwrap();

        let c1_idx = order.get(&c1).unwrap();
        let c2_idx = order.get(&c2).unwrap();

        if c1_idx > c2_idx {
            return true;
        }

        if c1_idx < c2_idx {
            return false;
        }

        idx += 1;
        continue;
    }
}
pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
    let c_idx_map: HashMap<char, usize> = order.char_indices().map(|(idx, v)| (v, idx)).collect();
    for idx in 1..words.len() {
        let s1 = words.get(idx).unwrap();
        let s2 = words.get(idx - 1).unwrap();

        if is_s1_bigger_than_s2(&c_idx_map, s1, s2) {
            continue;
        }

        return false;
    }

    return true;
}

fn main() {
    println!(
        "{}",
        is_alien_sorted(
            vec!["word".to_string(), "world".to_string(), "row".to_string()],
            "worldabcefghijkmnpqstuvxyz".to_string()
        )
    )
}
