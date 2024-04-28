fn dist(row1: i32, row2: i32, col1: i32, col2: i32) -> i32 {
    return (row1 - row2).abs() + (col1 - col2).abs();
}

pub fn all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>> {
    let max_dist = r_center.max(rows - r_center - 1) + c_center.max(cols - c_center - 1);
    let mut dists: Vec<Vec<Vec<i32>>> = vec![Vec::new(); (max_dist + 1) as usize];

    for i in 0..rows {
        for j in 0..cols {
            let point = vec![i, j];
            let dis = dist(i, r_center, j, c_center);
            dists[dis as usize].push(point);
        }
    }

    dists.concat()
}

fn main() {
    println!("{:?}", all_cells_dist_order(2, 3, 1, 2));
}
