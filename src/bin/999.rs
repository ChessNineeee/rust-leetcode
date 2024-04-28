pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
    let mut rook_row = 0;
    let mut rook_col = 0;
    let rows = board.len();
    let cols = board.get(0).unwrap().len();

    for (row_idx, row) in board.iter().enumerate() {
        for (col_idx, val) in row.iter().enumerate() {
            if *val == 'R' {
                rook_row = row_idx;
                rook_col = col_idx;
                break;
            }
        }
    }

    let mut res = 0;
    let mut row = rook_row;
    let mut col = rook_col;

    loop {
        if row == usize::MAX {
            break;
        }

        let face = board.get(row).unwrap().get(col).unwrap();
        if *face == 'B' {
            break;
        }

        if *face == 'p' {
            res += 1;
            break;
        }

        row = row.checked_sub(1).or(Some(usize::MAX)).unwrap();
    }

    row = rook_row;

    loop {
        if row == rows {
            break;
        }

        let face = board.get(row).unwrap().get(col).unwrap();
        if *face == 'B' {
            break;
        }

        if *face == 'p' {
            res += 1;
            break;
        }

        row = row.checked_add(1).or(Some(usize::MAX)).unwrap();
    }

    row = rook_row;

    loop {
        if col == usize::MAX {
            break;
        }

        let face = board.get(row).unwrap().get(col).unwrap();
        if *face == 'B' {
            break;
        }

        if *face == 'p' {
            res += 1;
            break;
        }

        col = col.checked_sub(1).or(Some(usize::MAX)).unwrap();
    }

    col = rook_col;

    loop {
        if col == cols {
            break;
        }

        let face = board.get(row).unwrap().get(col).unwrap();
        if *face == 'B' {
            break;
        }

        if *face == 'p' {
            res += 1;
            break;
        }

        col = col.checked_add(1).or(Some(usize::MAX)).unwrap();
    }
    res
}

fn main() {}
