fn is_palindrome(vec: &Vec<char>, first: usize, last: usize) -> bool {
    let mut first = first;
    let mut last = last;
    loop {
        if first >= last {
            break;
        }

        if vec[first] != vec[last] {
            return false;
        }

        first += 1;
        last -= 1;
    }
    return true;
}

fn valid_palindrome(s: String) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut j = chars.len() - 1;

    loop {
        if i >= j {
            break;
        }

        if chars[i] != chars[j] {
            return is_palindrome(&chars, i + 1, j) || is_palindrome(&chars, i, j - 1);
        }

        i += 1;
        j -= 1;
    }

    return true;
}

fn main() {
    println!("{}", valid_palindrome("deeee".to_string()));
}