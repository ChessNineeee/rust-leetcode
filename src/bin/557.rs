fn reverse_vec(vec: &mut Vec<char>, first: usize, last: usize) {
    let mut first = first;
    let mut last = last;

    loop {
        if first >= last {
            break;
        }

        vec.swap(first, last);
        first += 1;
        last -= 1;
    }
}

fn reverse_words(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut first = 0;
    let mut i = 1;
    let len = chars.len();

    loop {
        if i >= len {
            break;
        }

        if !chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        let last = i - 1;
        reverse_vec(&mut chars, first, last);

        i += 1;
        first = i;
    }

    let last = i - 1;
    reverse_vec(&mut chars, first, last);
    chars.into_iter().collect()
}

fn main() {
    println!("{}", reverse_words("Let's take LeetCode contest".to_string()));
}