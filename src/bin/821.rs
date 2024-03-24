pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
    let mut res = vec![10000; s.len()];
    let mut last_c_idx = -10000;

    for (idx, val) in s.chars().enumerate() {
        if val == c {
            last_c_idx = idx as i32;
            res[idx] = 0;
            continue;
        }

        res[idx] = res[idx].min((idx as i32 - last_c_idx).abs());
    }

    last_c_idx = -10000;
    for (idx, val) in s.chars().rev().enumerate() {
        let r_idx = s.len() - idx - 1;
        if val == c {
            last_c_idx = r_idx as i32;
            continue;
        }

        res[r_idx] = res[r_idx].min((r_idx as i32 - last_c_idx).abs());
    }
    res
}
fn main() {
    println!("{:?}", shortest_to_char("loveleetcode".to_string(), 'e'));
}
