pub fn max_area(height: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut left = 0;
    let mut right = height.len() - 1;
    loop {
        if left == right {
            break;
        }

        let board1 = height[left];
        let board2 = height[right];

        let area = board1.min(board2) * (right - left) as i32;
        res = res.max(area);

        if board1 < board2 {
            left += 1;
            continue;
        }

        right -= 1;
        continue;
    }
    res
}

fn main() {
    println!("{}", max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
}
