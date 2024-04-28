fn is_valid_col(strs: &Vec<String>, col_idx: usize) -> bool {
    let rows = strs.len();

    for row_idx in 1..rows {
        let c_at_row_idx_col_idx = strs.get(row_idx).unwrap().chars().nth(col_idx).unwrap();
        let c_at_row_idx_minus_1_col_idx =
            strs.get(row_idx - 1).unwrap().chars().nth(col_idx).unwrap();

        if c_at_row_idx_col_idx < c_at_row_idx_minus_1_col_idx {
            return false;
        }
    }

    true
}

pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let mut res = 0;
    let cols = strs.get(0).unwrap().len();

    for col_idx in 0..cols {
        if !is_valid_col(&strs, col_idx) {
            res += 1;
            continue;
        }
    }

    res
}
fn main() {
    println!(
        "{}",
        min_deletion_size(vec![
            "zyx".to_string(),
            "wvu".to_string(),
            "tsr".to_string()
        ])
    )
}
