fn area(blocks: i32) -> i32 {
    if blocks == 0 {
        return 0;
    }

    return blocks * 4 + 2;
}

fn area_to_sub(position: &(i32, i32), direction: &(i32, i32), grid: &Vec<Vec<i32>>) -> i32 {
    let rows = grid.len() as i32;
    let cols = grid.get(0).unwrap().len() as i32;

    let new_position = (position.0 + direction.0, position.1 + direction.1);
    if new_position.0 < 0 || new_position.0 >= rows {
        return 0;
    }

    if new_position.1 < 0 || new_position.1 >= cols {
        return 0;
    }

    let height = grid[position.0 as usize][position.1 as usize];
    let new_height = grid[new_position.0 as usize][new_position.1 as usize];

    if new_height >= height {
        return height;
    }

    new_height
}

pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    for (r_idx, row) in grid.iter().enumerate() {
        for (c_idx, val) in row.iter().enumerate() {
            res += area(*val);

            let position = (r_idx as i32, c_idx as i32);

            let left = (0, -1);
            res -= area_to_sub(&position, &left, &grid);

            let right = (0, 1);
            res -= area_to_sub(&position, &right, &grid);

            let up = (-1, 0);
            res -= area_to_sub(&position, &up, &grid);

            let down = (1, 0);
            res -= area_to_sub(&position, &down, &grid);
        }
    }
    res
}

fn main() {
    println!("{}", surface_area(vec![vec![1, 2], vec![3, 4]]));
}
