use std::collections::HashMap;

fn str_to_occur_map(str: &String) -> HashMap<char, i32> {
    let mut res = HashMap::new();
    for char in str.chars() {
        res.entry(char).and_modify(|v| *v += 1).or_insert(1);
    }
    res
}

fn intersection(m1: &HashMap<char, i32>, m2: &HashMap<char, i32>) -> HashMap<char, i32> {
    let mut res = HashMap::new();

    for (k, v) in m1.iter() {
        let entry = m2.get_key_value(k);
        if entry.is_some() {
            let (_, v2) = entry.unwrap();
            res.insert(*k, *v.min(v2));
            continue;
        }
    }

    res
}

fn map_to_vec(m2: &HashMap<char, i32>) -> Vec<String> {
    let mut res = Vec::new();
    for (k, v) in m2.iter() {
        for i in 0..*v {
            res.push(k.to_string());
        }
    }
    res
}

pub fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut inter = str_to_occur_map(words.get(0).unwrap());
    for word in words.iter().skip(1) {
        let map = str_to_occur_map(word);
        inter = intersection(&inter, &map);
    }

    return map_to_vec(&inter);
}

fn main() {
    println!(
        "{:?}",
        common_chars(vec![
            "bella".to_string(),
            "label".to_string(),
            "roller".to_string()
        ])
    )
}
