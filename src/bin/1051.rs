pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut count = vec![0; 101];

    for height in heights.iter() {
        count[*height as usize] += 1;
    }

    let mut presum = 0;
    for i in 1..count.len() {
        count[i] = presum + count[i];
        presum = count[i];
    }

    let mut res = 0;
    for (idx, height) in heights.into_iter().enumerate() {
        let start_idx = count[(height - 1) as usize];
        let end_idx = count[height as usize] - 1;

        if idx > end_idx || idx < start_idx {
            res += 1;
        }
    }

    res
}

fn main() {
    println!("{}", height_checker(vec![1, 1, 4, 2, 1, 3]));
}
