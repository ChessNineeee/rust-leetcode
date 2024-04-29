fn is_subsequence(s: String, t: String) -> bool {
    let len1 = s.len();
    let len2 = t.len();
    if len1 > len2 {
        return false;
    }

    let mut i = 0;
    let mut j = 0;

    let chars_s: Vec<char> = s.chars().collect();
    let chars_t: Vec<char> = t.chars().collect();

    loop {
        if i >= len1 {
            break;
        }

        if j >= len2 {
            break;
        }

        let char_s = chars_s[i];
        let char_t = chars_t[j];

        if char_s == char_t {
            i += 1;
            j += 1;
            continue;
        }

        j += 1;
    }

    return i == len1;
}

fn main() {
    println!("{}", is_subsequence("abc".to_string(), "acebxxec".to_string()));
}