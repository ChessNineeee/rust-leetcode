use std::collections::HashSet;

fn is_vowel(vowels: &HashSet<char>, c: char) -> bool {
    if !c.is_ascii_alphabetic() {
        return false;
    }

    if !vowels.contains(&c.to_ascii_lowercase()) {
        return false;
    }

    return true;
}
fn reverse_vowels(s: String) -> String {
    let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
    let mut i = 0;
    let mut j = s.len() - 1;
    let mut chars: Vec<char> = s.chars().collect();

    loop {
        if i >= j {
            break;
        }

        let char_i = chars[i];
        if !is_vowel(&vowels, char_i) {
            i += 1;
            continue;
        }

        let char_j = chars[j];
        if !is_vowel(&vowels, char_j) {
            j -= 1;
            continue;
        }

        chars.swap(i, j);
        i += 1;
        j -= 1;
    }

    chars.into_iter().collect()
}
fn main() {
    println!("{}", reverse_vowels("leetcode".to_string()));
}