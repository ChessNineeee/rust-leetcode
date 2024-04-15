pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
    let sum: i32 = arr.iter().sum();

    if sum % 3 != 0 {
        return false;
    }

    let mut idx = 0;
    let mut cur_sum = 0;
    let mut first_part = false;
    let mut second_part = false;
    let mut second_part_idx = usize::MAX;

    loop {
        if idx == arr.len() {
            break;
        }

        cur_sum += arr[idx];

        if cur_sum * 3 / 2 == sum && first_part {
            second_part = true;
            second_part_idx = idx;
            break;
        }

        if cur_sum * 3 == sum {
            first_part = true;
        }

        idx += 1;
    }
    return first_part && second_part && second_part_idx != arr.len() - 1;
}

fn main() {
    println!(
        "{}",
        can_three_parts_equal_sum(vec![18, 12, -18, 18, -19, -1, 10, 10])
    );
}
