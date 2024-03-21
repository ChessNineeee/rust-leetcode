use std::collections::HashSet;

pub fn find_words(words: Vec<String>) -> Vec<String> {
    let line1 = "qwertyuiop";
    let line2 = "asdfghjkl";
    let line3 = "zxcvbnm";

    let line1: HashSet<char> = line1.chars().into_iter().collect();
    let line2: HashSet<char> = line2.chars().into_iter().collect();
    let line3: HashSet<char> = line3.chars().into_iter().collect();

    let mut res = Vec::new();

    for word in words.into_iter() {
        let c_set: HashSet<char> = word.to_lowercase().chars().into_iter().collect();
        if c_set.is_subset(&line1) || c_set.is_subset(&line2) || c_set.is_subset(&line3) {
            res.push(word);
        }
    }
    res
}

fn main() {
    println!(
        "{:?}",
        find_words(vec![
            String::from("Hello"),
            String::from("Alaska"),
            String::from("Dad")
        ])
    )
}
