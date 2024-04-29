fn get_next(p: &String) -> Vec<usize> {
    let len = p.len();
    let mut next = vec![0; len + 1];

    let mut j = 2;
    let mut i = 0;
    loop {
        if j > len {
            break;
        }

        let char_i = p.chars().nth(i).unwrap();
        let char_j_1 = p.chars().nth(j - 1).unwrap();

        if char_i == char_j_1 {
            i += 1;
            next[j] = i;
            j += 1;
            continue;
        }

        if i == 0 {
            next[j] = i;
            j += 1;
            continue;
        }

        i = next[i];
    }
    next
}

fn str_str(haystack: String, needle: String) -> i32 {
    let next = get_next(&needle);
    let mut i = 0;
    let mut j = 0;
    let h_len = haystack.len();
    let n_len = needle.len();

    loop {
        if j == n_len {
            return (i - n_len) as i32;
        }
        
        if i == h_len {
            break;
        }

        let char_i = haystack.chars().nth(i).unwrap();
        let char_j = needle.chars().nth(j).unwrap();

        if char_i == char_j {
            i += 1;
            j += 1;
            continue;
        }

        if j == 0 {
            i += 1;
            continue;
        }

        j = next[j];
    }

    -1
}

fn main() {
    println!(
        "{}",
        str_str(String::from("leetcode"), String::from("leeto"))
    )
}
