use std::collections::HashSet;

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    let morse_table = [
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];

    words
        .into_iter()
        .map(|word| {
            word.chars().fold(String::new(), |string, char| {
                let idx = char as usize - 'a' as usize;
                string + morse_table[idx]
            })
        })
        .collect::<HashSet<String>>()
        .len() as i32
}
fn main() {
    println!(
        "{}",
        unique_morse_representations(vec![
            "gin".to_string(),
            "zen".to_string(),
            "gig".to_string(),
            "msg".to_string()
        ])
    )
}
