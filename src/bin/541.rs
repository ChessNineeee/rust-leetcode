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

fn reverse_str(s: String, k: i32) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut first = 0;
    let len = chars.len();
    loop {
        if first >= len {
            break;
        }

        let mut last = first + k as usize - 1;
        if last >= len {
            last = len - 1;
        }

        reverse_vec(&mut chars, first, last);
        first += 2 * k as usize;
    }
    chars.into_iter().collect()
}

fn main() {
    println!("{}", reverse_str("abcdefg".to_string(), 2));
}