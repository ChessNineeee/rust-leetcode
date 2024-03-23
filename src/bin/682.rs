pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut scores = Vec::<i32>::new();

    for op in operations.into_iter() {
        if op == "+" {
            let last = *scores.last().unwrap();
            let second_last = *scores.iter().nth_back(1).unwrap();
            scores.push(last + second_last);
            continue;
        }

        if op == "C" {
            scores.pop();
            continue;
        }

        if op == "D" {
            let last = *scores.last().unwrap();
            scores.push(last * 2);
            continue;
        }

        let score = i32::from_str_radix(&op, 10).unwrap();
        scores.push(score);
    }

    scores.into_iter().sum::<i32>()
}
fn main() {
    println!(
        "{}",
        cal_points(vec![
            "5".to_string(),
            "2".to_string(),
            "C".to_string(),
            "D".to_string(),
            "+".to_string()
        ])
    )
}
