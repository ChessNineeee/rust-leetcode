pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
    let mut lines = 0;
    let mut width_sum = 0;
    for char in s.chars() {
        let width_idx = char as usize - 'a' as usize;
        let width = widths[width_idx];

        let new_width_sum = width_sum + width;
        if new_width_sum > 100 {
            lines += 1;
            width_sum = width;
        } else {
            width_sum = new_width_sum;
        }
    }

    if width_sum != 0 {
        lines += 1;
    }

    vec![lines, width_sum]
}
fn main() {
    println!(
        "{:?}",
        number_of_lines(
            vec![
                10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10
            ],
            "abcdefghijklmnopqrstuvwxyz".to_string()
        )
    )
}
