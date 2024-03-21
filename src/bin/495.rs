pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
    let mut old_left = -1;
    let mut old_right = -1;

    let mut res = 0;

    for it in time_series.iter() {
        let new_left = *it;
        let new_right = *it + duration - 1;

        if new_left > old_right {
            old_left = new_left;
            old_right = new_right;
            res += duration;
            continue;
        }

        res += (new_right - old_right);
        old_right = new_right;
    }

    return res;
}

fn main() {
    println!("{}", find_poisoned_duration(vec![1, 2,3,4,5,6,7,8,9], 10));
}
