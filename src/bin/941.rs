pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
    let len = arr.len();
    if len < 3 {
        return false;
    }

    let mut peak_count = 0;
    for idx in 1..len - 1 {
        let is_up_slope = arr[idx] < arr[idx + 1] && arr[idx] > arr[idx - 1];
        if is_up_slope {
            continue;
        }

        let is_down_slope = arr[idx] > arr[idx + 1] && arr[idx] < arr[idx - 1];
        if is_down_slope {
            continue;
        }

        let is_peak = arr[idx] > arr[idx + 1] && arr[idx] > arr[idx - 1];
        if is_peak {
            peak_count += 1;
            continue;
        }

        return false;
    }

    peak_count == 1
}

fn main() {
    println!("{}", valid_mountain_array(vec![3, 5, 5]))
}
