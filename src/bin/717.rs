pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
    let small_step = 1;
    let big_step = 2;

    let mut idx = 0;
    let mut last_action = 0;
    loop {
        if idx == bits.len() {
            break;
        }

        match bits[idx] {
            0 => {
                idx += small_step;
                last_action = small_step;
                continue;
            }
            1 => {
                idx += big_step;
                last_action = big_step;
                continue;
            }
            default => {
                unreachable!("never other value: {}", default)
            }
        }
    }

    last_action == small_step
}
fn main() {
    println!("{}", is_one_bit_character(vec![1, 0, 1, 0]))
}
